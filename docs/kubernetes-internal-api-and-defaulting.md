# Kubernetes Internal API 架构与 Defaulting 机制

## 概述

本文档详细分析 Kubernetes API Server 如何利用 Internal API 进行请求处理，以及 Defaulting、Conversion、Validation 在不同 API 版本上的执行机制。

---

## 1. 三层版本架构

Kubernetes 使用三层版本架构处理 API 请求：

```
┌──────────────────┐
│  External API    │  客户端请求版本 (v1, v1beta1, v1beta2, ...)
│  (v1/v1beta1)    │  - 面向用户的稳定 API
└────────┬─────────┘  - 每个版本可能有不同的字段、默认值
         │
         │ Decode + Defaulting
         ▼
┌──────────────────┐
│  Hub Version     │  内部处理版本 (通常是 internal)
│  (internal)      │  - 业务逻辑的统一执行环境
└────────┬─────────┘  - Validation、Admission Control 在此执行
         │
         │ Encode (via EncodeVersioner)
         ▼
┌──────────────────┐
│ Storage Version  │  持久化版本 (通常是最新稳定版 v1)
│  (v1)            │  - 存储到 etcd 的实际格式
└──────────────────┘  - 保证存储格式的稳定性
```

### 关键设计原则

1. **External API ≠ Storage Format**
   - 客户端使用的 API 版本可以与存储版本不同
   - 例如：客户端用 v1beta1，内部转为 internal 处理，存储用 v1

2. **Internal API 是中间格式**
   - Internal 永远不会直接暴露给客户端
   - Internal 永远不会直接存储到 etcd
   - Internal 是所有版本转换的"枢纽"（Hub）

3. **版本转换的星型拓扑**
   ```
   v1 ←→ internal ←→ v1beta1
          ↕
        v1beta2

   只需实现 n 个转换，而不是 n(n-1)/2 个
   ```

---

## 2. Defaulting 在 External Version 执行

### 2.1 源码证据

**External Version 有 Defaulting**

文件: `k8s/pkg/apis/core/v1/defaults.go`
```go
// SetDefaults_Pod sets defaults for v1.Pod
func SetDefaults_Pod(obj *v1.Pod) {
    for i := range obj.Spec.Containers {
        SetDefaults_Container(&obj.Spec.Containers[i])
    }
    SetDefaults_PodSpec(&obj.Spec)
}

func SetDefaults_Container(obj *v1.Container) {
    if obj.ImagePullPolicy == "" {
        // v1 specific default
        obj.ImagePullPolicy = v1.PullIfNotPresent
    }
    if obj.TerminationMessagePath == "" {
        obj.TerminationMessagePath = v1.TerminationMessagePathDefault
    }
    if obj.TerminationMessagePolicy == "" {
        obj.TerminationMessagePolicy = v1.TerminationMessageReadFile
    }
}
```

**Internal Version 没有 Defaulting**

目录结构:
```
pkg/apis/core/
├── types.go                 # Internal type definitions
├── validation/              # Validation only
│   └── validation.go
├── register.go
└── ... (NO defaults.go!)    # Internal version 没有 defaulting
```

### 2.2 执行时机

**文件**: `staging/src/k8s.io/apimachinery/pkg/runtime/serializer/versioning/versioning.go`

```go
func (c *codec) Decode(data []byte, defaultGVK *schema.GroupVersionKind, into runtime.Object) {
    // 步骤 1: 反序列化为 external version
    obj, gvk, err := c.decoder.Decode(data, defaultGVK, nil)
    // obj 现在是 v1::Pod (未设置默认值)

    // 步骤 2: 对 external version 执行 defaulting
    if c.defaulter != nil {
        c.defaulter.Default(obj)  // obj 是 v1::Pod，在此设置默认值
    }

    // 步骤 3: 转换为 internal version
    if err := c.convertor.Convert(obj, into, c.decodeVersion); err != nil {
        return nil, gvk, err
    }
    // into 现在是 internal::Pod (已包含从 v1 传递来的默认值)

    return into, gvk, nil
}
```

**关键流程**:
```
JSON bytes
  ↓ Decode
v1::Pod (无默认值)
  ↓ Default()
v1::Pod (有默认值: imagePullPolicy="IfNotPresent", restartPolicy="Always")
  ↓ Convert()
internal::Pod (包含从 v1 传递的默认值)
```

### 2.3 为什么在 External Version 做 Defaulting？

#### 原因 1: 默认值是 API 契约的一部分

不同 API 版本可能有不同的默认值：

```go
// v1/defaults.go
func SetDefaults_Service(obj *v1.Service) {
    if obj.Spec.SessionAffinity == "" {
        obj.Spec.SessionAffinity = v1.ServiceAffinityNone  // v1 默认
    }
}

// 假设 v1beta1 有不同的默认值
func SetDefaults_Service(obj *v1beta1.Service) {
    if obj.Spec.SessionAffinity == "" {
        obj.Spec.SessionAffinity = v1beta1.ServiceAffinityClientIP  // 不同的默认值
    }
}
```

**API 演进需求**：
- 新版本可能改变默认值（非兼容性变更）
- 旧版本的客户端期望旧的默认值
- 默认值是 OpenAPI schema 的明确部分

#### 原因 2: Internal Version 应该是纯净的

```go
// Internal version 的设计哲学
type Pod struct {
    // 不应该包含版本特定的行为
    // 字段结构可能与任何 external version 都不同
    Spec PodSpec
    Status PodStatus
}
```

Internal version 特点：
- 是所有 external version 的抽象超集
- 不应该有版本特定的逻辑（包括默认值）
- 只负责统一表示和业务逻辑执行

#### 原因 3: Defaulting 的注册机制

**文件**: `pkg/apis/core/v1/zz_generated.defaults.go`
```go
func RegisterDefaults(scheme *runtime.Scheme) error {
    // 为每个 v1 类型注册 defaulting 函数
    scheme.AddTypeDefaultingFunc(&corev1.Pod{}, func(obj interface{}) {
        SetObjectDefaults_Pod(obj.(*corev1.Pod))
    })
    scheme.AddTypeDefaultingFunc(&corev1.Service{}, func(obj interface{}) {
        SetObjectDefaults_Service(obj.(*corev1.Service))
    })
    // ... 更多类型
    return nil
}
```

**运行时调度**:
```go
// runtime/scheme.go
func (s *Scheme) Default(obj Object) {
    // 通过 reflect.TypeOf 查找对应的 defaulting 函数
    if fn, ok := s.defaulterFuncs[reflect.TypeOf(obj)]; ok {
        fn(obj)  // 调用 SetObjectDefaults_Pod(obj.(*v1.Pod))
    }
}
```

只有 external version 注册�� defaulting 函数，internal version 没有。

---

## 3. Validation 在 Internal Version 执行

### 3.1 Validation 实现

**文件**: `pkg/apis/core/validation/validation.go`

```go
// 只有一套验证逻辑，针对 internal version
func ValidatePod(pod *core.Pod) field.ErrorList {
    // pod 是 internal::Pod
    allErrs := ValidateObjectMeta(&pod.ObjectMeta, true, ValidatePodName, ...)
    allErrs = append(allErrs, ValidatePodSpecificAnnotations(pod.Annotations, &pod.Spec, ...)...)
    allErrs = append(allErrs, ValidatePodSpec(&pod.Spec, &pod.ObjectMeta, ...)...)
    return allErrs
}

func ValidatePodSpec(spec *core.PodSpec, podMeta *metav1.ObjectMeta, ...) field.ErrorList {
    specPath := field.NewPath("spec")
    allErrs := field.ErrorList{}

    // 验证容器列表不为空
    if len(spec.Containers) == 0 {
        allErrs = append(allErrs, field.Required(specPath.Child("containers"), "must have at least one container"))
    }

    // 验证每个容器
    for i, ctr := range spec.Containers {
        allErrs = append(allErrs, ValidateContainer(&ctr, spec, podMeta, specPath.Child("containers").Index(i), ...)...)
    }

    return allErrs
}
```

### 3.2 为什么 Validation 在 Internal Version？

#### 合法性规则应该版本无关

```go
// 不合法的 Pod 在任何版本都应该不合法
v1::Pod { spec: { containers: [] } }           // 不合法
v1beta1::Pod { spec: { containers: [] } }      // 不合法
internal::Pod { spec: { containers: [] } }     // 不合法
```

**设计原则**：
- 对象"合法"与否不应该因 API 版本而改变
- Validation 是业务规则，不是 API 契约
- 在 internal version 验证，避免每个版本重复代码

#### Validation 调用链

**文件**: `registry/generic/registry/store.go`
```go
func (e *Store) Create(ctx context.Context, obj runtime.Object, ...) (runtime.Object, error) {
    // obj 已经是 internal version

    // 策略验证 (在 internal version 上)
    if errs := e.CreateStrategy.Validate(ctx, obj); len(errs) > 0 {
        return nil, apierrors.NewInvalid(qualifiedResource, name, errs)
    }

    // 调用存储层
    out := e.NewFunc()
    if err := e.Storage.Create(ctx, key, obj, out, ttl, dryrun); err != nil {
        return nil, err
    }

    return out, nil
}
```

**文件**: `registry/core/pod/strategy.go`
```go
type podStrategy struct {
    runtime.ObjectTyper
    names.NameGenerator
}

func (podStrategy) Validate(ctx context.Context, obj runtime.Object) field.ErrorList {
    pod := obj.(*api.Pod)  // api.Pod 是 internal version

    // 调用 internal validation
    opts := validation.ValidationOptionsForPod(pod, nil)
    return validation.ValidatePod(pod, opts)
}
```

---

## 4. Conversion 机制

### 4.1 双向转换实现

**手动转换函数** (处理特殊逻辑):

文件: `pkg/apis/core/v1/conversion.go`
```go
func Convert_v1_Pod_To_core_Pod(in *v1.Pod, out *core.Pod, s conversion.Scope) error {
    // 调用自动生成的基础转换
    if err := autoConvert_v1_Pod_To_core_Pod(in, out, s); err != nil {
        return err
    }

    // 自定义转换逻辑
    // 例如：清理已废弃的注解
    out.Annotations = dropInitContainerAnnotations(out.Annotations)

    return nil
}

func Convert_core_Pod_To_v1_Pod(in *core.Pod, out *v1.Pod, s conversion.Scope) error {
    if err := autoConvert_core_Pod_To_v1_Pod(in, out, s); err != nil {
        return err
    }

    // 反向转换的自定义逻辑
    return nil
}
```

**自动生成的转换** (字段对字段复制):

文件: `pkg/apis/core/v1/zz_generated.conversion.go`
```go
func autoConvert_v1_Pod_To_core_Pod(in *corev1.Pod, out *core.Pod, s conversion.Scope) error {
    out.ObjectMeta = in.ObjectMeta

    if err := Convert_v1_PodSpec_To_core_PodSpec(&in.Spec, &out.Spec, s); err != nil {
        return err
    }

    if err := Convert_v1_PodStatus_To_core_PodStatus(&in.Status, &out.Status, s); err != nil {
        return err
    }

    return nil
}
```

### 4.2 Conversion 注册

**文件**: `pkg/apis/core/v1/register.go`
```go
func init() {
    localSchemeBuilder.Register(addDefaultingFuncs, addConversionFuncs)
}

func addConversionFuncs(scheme *runtime.Scheme) error {
    return RegisterConversions(scheme)
}
```

**文件**: `pkg/apis/core/v1/zz_generated.conversion.go`
```go
func RegisterConversions(s *runtime.Scheme) error {
    if err := s.AddConversionFunc((*corev1.Pod)(nil), (*core.Pod)(nil),
        func(a, b interface{}, scope conversion.Scope) error {
            return Convert_v1_Pod_To_core_Pod(a.(*corev1.Pod), b.(*core.Pod), scope)
        }); err != nil {
        return err
    }

    if err := s.AddConversionFunc((*core.Pod)(nil), (*corev1.Pod)(nil),
        func(a, b interface{}, scope conversion.Scope) error {
            return Convert_core_Pod_To_v1_Pod(a.(*core.Pod), b.(*corev1.Pod), scope)
        }); err != nil {
        return err
    }

    // ... 更多转换函数
    return nil
}
```

---

## 5. 完整请求处理流程

### 5.1 CREATE 请求流程

```
┌─────────────────────────────────────────────────┐
│ 1. HTTP Request                                 │
│    POST /api/v1/namespaces/default/pods         │
│    Body: { "apiVersion": "v1", "kind": "Pod",   │
│            "spec": { "containers": [...] } }    │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 2. HTTP Handler (create.go:125-127)             │
│    decoder := scope.Serializer.DecoderToVersion │
│    obj, gvk := decoder.Decode(body)             │
│    Result: v1::Pod (with defaults)              │
└─────────────────┬───────────────────────────────┘
                  │
                  │ (Defaulting happens in Decode!)
                  ▼
┌─────────────────────────────────────────────────┐
│ 3. Conversion (versioning.go)                   │
│    internal_pod = Convert_v1_Pod_To_core_Pod()  │
│    Result: internal::Pod                        │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 4. Validation (store.go:477-558)                │
│    errs := e.CreateStrategy.Validate(internal)  │
│    Calls: validation.ValidatePod(internal::Pod) │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 5. Admission Control                            │
│    Mutating/Validating webhooks on internal     │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 6. Storage Encoding (etcd3/store.go:532-538)    │
│    data = runtime.Encode(s.codec, internal_pod) │
│    Codec uses EncodeVersioner to select v1      │
│    Result: v1::Pod JSON bytes                   │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 7. Transformation (optional encryption)         │
│    newData = s.transformer.TransformToStorage() │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│ 8. etcd Write                                   │
│    Key: /registry/core/v1/pods/default/nginx    │
│    Value: (encrypted) v1::Pod JSON              │
│    Lease: TTL management                        │
│    CAS: ResourceVersion optimistic locking      │
└─────────────────────────────────────────────────┘
```

### 5.2 关键代码路径

| 阶段 | 文件 | 行号 | 操作 |
|------|------|------|------|
| HTTP 解码 | `handlers/create.go` | 125-127 | Decode + Defaulting |
| 转换到 internal | `serializer/versioning/versioning.go` | 169-173 | v1 → internal |
| 验证 | `registry/store.go` | 477-558 | Validate internal |
| 存储编码 | `etcd3/store.go` | 532-538 | internal → v1 JSON |
| etcd 写入 | `etcd3/store.go` | 583-586 | OptimisticPut |

---

## 6. Storage 层详解

### 6.1 Codec 与 EncodeVersioner

**文件**: `storage/storagebackend/config.go`
```go
type Config struct {
    // Codec 负责编码/解码
    Codec runtime.Codec

    // EncodeVersioner 决定存储版本
    // 给定 internal GVK，返回目标存储 GV
    EncodeVersioner runtime.GroupVersioner

    // Transformer 处理加密/压缩
    Transformer value.Transformer
}
```

**EncodeVersioner 工作原理**:
```go
// 示例：PreferredStorageVersioner
type PreferredStorageVersioner struct {
    Preferred schema.GroupVersion  // 例如 {Group: "", Version: "v1"}
}

func (v *PreferredStorageVersioner) KindForGroupVersionKinds(kinds []schema.GroupVersionKind) (schema.GroupVersionKind, bool) {
    // 输入: {Group: "", Version: "__internal", Kind: "Pod"}
    // 输出: {Group: "", Version: "v1", Kind: "Pod"}
    return schema.GroupVersionKind{
        Group:   v.Preferred.Group,
        Version: v.Preferred.Version,
        Kind:    kinds[0].Kind,
    }, true
}
```

### 6.2 Storage Key 结构

**生成函数**: `registry/generic/registry/store.go:268-307`

```go
func NamespaceKeyFunc(ctx context.Context, prefix string, name string) (string, error) {
    namespace, ok := genericapirequest.NamespaceFrom(ctx)
    if !ok || len(namespace) == 0 {
        return "", apierrors.NewBadRequest("Namespace parameter required.")
    }
    // 结果示例: /registry/core/v1/pods/default/nginx
    return prefix + "/" + namespace + "/" + name, nil
}
```

**Key 结构**:
```
/registry                     <- 固定前缀
  /core                       <- API Group (core 是空字符串的映射)
    /v1                       <- API Version (存储版本，不是 internal)
      /pods                   <- Resource 类型
        /default              <- Namespace
          /nginx              <- 资源名称
```

**注意**: Key 中包含版本信息 (v1)，但这只是路径约定，实际对象在 value 中以 JSON 存储。

### 6.3 ResourceVersion 与乐观锁

**Versioner 接口**: `storage/interfaces.go`
```go
type Versioner interface {
    // 从 etcd revision 更新到对象的 metadata.resourceVersion
    UpdateObject(obj runtime.Object, resourceVersion uint64) error

    // 从对象提取 resourceVersion
    ObjectResourceVersion(obj runtime.Object) (uint64, error)

    // 存储前清理 resourceVersion (由 etcd 生成)
    PrepareObjectForStorage(obj runtime.Object) error
}
```

**乐观锁流程**:
```go
// etcd3/store.go:583-603
txnResp, err := s.client.Kubernetes.OptimisticPut(
    ctx,
    key,
    newData,
    origState.rev,  // ← 前一次读取的 ResourceVersion
    kubernetes.PutOptions{...},
)

if !txnResp.Succeeded {
    // 冲突：对象在读取后被其他客户端修改
    // 重试：重新读取，重新执行 update 逻辑
}
```

**UPDATE 冲突处理**:
```go
// registry/store.go:617-822
err = e.Storage.GuaranteedUpdate(ctx, key, out, ignoreNotFound, storagePreconditions,
    func(existing runtime.Object, res storage.ResponseMeta) (runtime.Object, *uint64, error) {
        // 这个回调可能被调用多次（冲突时重试）

        obj, err := objInfo.UpdatedObject(ctx, existing)

        // 检查 ResourceVersion 冲突
        newResourceVersion, _ := e.Storage.Versioner().ObjectResourceVersion(obj)
        if newResourceVersion != existingResourceVersion {
            return nil, nil, apierrors.NewConflict(...)
        }

        return obj, &ttl, nil
    },
    dryrun,
    nil,
)
```

---

## 7. 对比总结

### 7.1 Defaulting vs Validation vs Conversion

| 特性 | Defaulting | Validation | Conversion |
|------|------------|------------|------------|
| **执行版本** | External (v1/v1beta1) | Internal | External ↔ Internal |
| **实现位置** | `pkg/apis/core/v1/defaults.go` | `pkg/apis/core/validation/` | `pkg/apis/core/v1/conversion.go` |
| **函数签名** | `SetDefaults_Pod(*v1.Pod)` | `ValidatePod(*core.Pod)` | `Convert_v1_Pod_To_core_Pod()` |
| **版本相关性** | 是（每个版本可能不同） | 否（版本无关） | 是（连接两个版本） |
| **执行时机** | Decode 阶段 | Registry 层 | Decode/Encode 阶段 |
| **是否幂等** | 是 | 是（只读） | 是 |

### 7.2 版本流转总结

```
Client Request (v1 JSON)
  ↓ Deserialize
v1::Pod (raw)
  ↓ ApplyDefaulting
v1::Pod (defaulted)           ← Defaulting 在这里
  ↓ Convert (v1 → internal)
internal::Pod                 ← Validation 在这里
  ↓ Convert (internal → v1)   ← EncodeVersioner 决定目标版本
v1::Pod (for storage)
  ↓ Serialize to JSON
v1::Pod JSON bytes
  ↓ Optional encryption
Encrypted bytes
  ↓ Write to etcd
etcd: /registry/core/v1/pods/default/nginx = (encrypted v1 JSON)
```

---

## 8. taibai_api 实现验证

### 8.1 当前实现是正确的

**taibai_api 的设计完全符合 Kubernetes 上游**:

```rust
// taibai_api/src/core/v1/pod.rs
impl ApplyDefaulting for Pod {
    fn apply_defaulting(&mut self, path: &Path) {
        // ✅ 在 v1::Pod 上实现 defaulting (正确)
        if let Some(spec) = &mut self.spec {
            spec.apply_defaulting(&path.field("spec"));
        }
    }
}

// taibai_api/src/core/internal/pod.rs
// ✅ 没有 ApplyDefaulting 实现 (正确)
```

### 8.2 Harness 实现验证

```rust
// taibai_api/src/harness/helpers.rs:28-52
pub fn make_default_handler<T>(gvk: &'static str) -> ...
where
    T: DeserializeOwned + ApplyDefaulting + Serialize + 'static,
{
    Box::new(move |json| {
        let mut obj: T = parse_json(json)?;  // T = v1::Pod
        obj.apply_defaulting(&Path::root());  // ✅ 在 v1 上 defaulting
        let result = to_value(&obj)?;
        Ok(DefaultResult {
            gvk: gvk.to_string(),
            result,
            defaults_applied: true,
        })
    })
}
```

**对应 Kubernetes 的流程**:
```go
// versioning.go
obj, _, _ := c.decoder.Decode(data, ...)  // 对应 parse_json
c.defaulter.Default(obj)                  // 对应 apply_defaulting
```

### 8.3 Fixtures 验证正确性

**taibai_api_fixtures 生成流程**:
```go
// cmd/generate/internal/fixtures/defaulting.go:30-67
func generateDefault(name, comment string, obj runtime.Object) (*Fixture, error) {
    gvk := gvkFor(obj)  // obj 是 v1::Pod

    // 1. 序列化输入 (未 defaulting 的 v1)
    inputJSON, _ := normalizeJSON(json.MarshalIndent(obj))

    // 2. 对 v1 对象执行 defaulting
    defaulted := obj.DeepCopyObject()
    legacyscheme.Scheme.Default(defaulted)  // ← 在 v1 上 defaulting

    // 3. 序列化输出 (已 defaulting 的 v1)
    outputJSON, _ := normalizeJSON(json.MarshalIndent(defaulted))

    return &Fixture{
        Input:  inputJSON,   // v1 input
        Output: outputJSON,  // v1 output (with defaults)
    }
}
```

**验证工具比较**:
```go
// cmd/verify/main.go:371-431
result, err := client.ApplyDefaults(gvk, fixture.Input)
// result.Result 是 taibai_api 的 v1::Pod (defaulted)

diffResult, err := diff.Compare(fixture.Output, result.Result)
// 比较 Kubernetes v1::Pod vs taibai_api v1::Pod
```

---

## 9. 关键设计洞察

### 9.1 为什么 Internal API 不是万能的

**Internal API 的限制**:
1. 不能直接暴露给客户端（没有稳定性保证）
2. 不能直接存储到 etcd（格式可能频繁变动）
3. 不应该包含版本特定的逻辑（保持纯净）

**Internal API 的优势**:
1. 统一业务逻辑执行环境（validation, admission）
2. 简化版本转换（星型拓扑）
3. 解耦 API 演进和存储格式

### 9.2 三层架构的必要性

```
External API ─┬─ 面向客户端的稳定接口
              └─ 版本特定的默认值和行为

Internal API ─┬─ 业务逻辑的统一执行环境
              └─ 所有版本的抽象超集

Storage API ──┬─ 持久化格式的稳定性
              └─ 数据迁移和兼容性保证
```

每一层都有明确的职责，缺一不可。

### 9.3 Codec 是核心组件

```
Codec = Serializer + Deserializer + Defaulter + Convertor
```

**Codec 职责**:
- Deserialize JSON → External type
- Apply defaults on External type
- Convert External → Internal
- Validate Internal type
- Convert Internal → Storage version
- Serialize Storage type → JSON

**taibai_api 需要实现的 Codec 能力**:
```rust
trait Codec {
    // Decode: JSON → v1 → defaulting → internal
    fn decode(&self, data: &[u8]) -> Result<Box<dyn Object>>;

    // Encode: internal → storage version → JSON
    fn encode(&self, obj: &dyn Object) -> Result<Vec<u8>>;
}
```

---

## 10. 文件路径速查

### 10.1 Defaulting 相关

| 描述 | 路径 |
|------|------|
| v1 Defaulting 手写函数 | `pkg/apis/core/v1/defaults.go` |
| v1 Defaulting 生成注册 | `pkg/apis/core/v1/zz_generated.defaults.go` |
| Defaulting 注册入口 | `pkg/apis/core/v1/register.go` |
| Scheme Default 实现 | `staging/.../apimachinery/pkg/runtime/scheme.go` |

### 10.2 Conversion 相关

| 描述 | 路径 |
|------|------|
| v1↔internal 手写转换 | `pkg/apis/core/v1/conversion.go` |
| v1↔internal 自动转换 | `pkg/apis/core/v1/zz_generated.conversion.go` |
| Versioning Codec | `staging/.../runtime/serializer/versioning/versioning.go` |

### 10.3 Validation 相关

| 描述 | 路径 |
|------|------|
| Pod Validation | `pkg/apis/core/validation/validation.go` |
| Validation 策略 | `pkg/registry/core/pod/strategy.go` |

### 10.4 Storage 相关

| 描述 | 路径 |
|------|------|
| Generic Registry | `staging/.../apiserver/pkg/registry/generic/registry/store.go` |
| etcd3 Storage | `staging/.../apiserver/pkg/storage/etcd3/store.go` |
| Storage Config | `staging/.../apiserver/pkg/storage/storagebackend/config.go` |
| Versioner 接口 | `staging/.../apiserver/pkg/storage/interfaces.go` |

---

## 11. 总结

### 核心理解

1. **Defaulting 在 External Version 执行**
   - 默认值是 API 契约的一部分
   - 不同版本可能有不同默认值
   - Codec 在 Decode 阶段自动应用

2. **Validation 在 Internal Version 执行**
   - 合法性规则版本无关
   - 只需维护一套验证逻辑
   - Registry 层调用

3. **Conversion 连接两个世界**
   - External ↔ Internal 双向转换
   - 星型拓扑简化版本管理
   - 手写 + 自动生成结合

4. **Storage Version 保证持久化稳定性**
   - Internal 不直接存储
   - EncodeVersioner 选择目标版本
   - etcd 中存储稳定的 External version

### taibai_api 的正确性

taibai_api 的实现完全遵循了 Kubernetes 上游的设计：
- ✅ `ApplyDefaulting` 在 External version (v1) 实现
- ✅ `ToInternal/FromInternal` 提供双向转换
- ✅ Harness 在 v1 上执行 defaulting
- ✅ Fixtures 比较 v1 的 defaulting 结果

这是对 Kubernetes 架构的正确理解和实现。
