# HIR Node 구조서

## 역할

HIR은 AST를 의미 분석용 Canonical 구조로 바꾼다.

```text
AST → HIR → Resolution/Type Tables → Typed HIR → MIR
```

## ID

```text
HirId HirItemId HirExprId HirStmtId
HirPatternId ScopeId DefId LocalId
```

## 정규화

- `T?` → `Option<T>`
- `void`·생략 반환 → Unit
- Unit 값 → `()`
- `try`, `exists`, `using`, `for`는 전용 HIR로 보존
- 이름 인수는 SourceOrder와 Label을 보존
- 할당은 HirStatement

## Side Table

```text
ResolutionMap<HirId, Resolution>
TypeTable<HirId, TypeId>
CoercionTable<HirId, Coercion>
OwnershipTable<HirId, OwnershipAction>
EffectTable<DefId, Effects>
```

## 원칙

- HIR은 Trivia에 의존하지 않음
- Synthetic Node는 SourceOrigin 보존
- Error HIR로 후속 분석 지속
