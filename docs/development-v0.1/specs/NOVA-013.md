# NOVA-013 — 줄바꿈·문장 종료·연속 줄 규칙

| 항목 | 값 |
|---|---|
| 대상 | Nova 0.1 |
| 작성일 | 2026-10-03 |
| 문서 상태 | Draft — 기존 결정은 유지, 추가 상세는 승인 대기 |
| 적용 Stage | A~B |
| 근거 | [원본 NOVA-013](../../01_Source_Syntax/NOVA-013_줄바꿈_문장_종료_연속_줄_규칙.md) |

이 문서는 원본을 대체하는 확정 사양이 아니다. 기존 확정 기준은 [CANONICAL](../CANONICAL.md)을 따르며, 새 의미·문법·API·정책은 [DECISIONS](../DECISIONS.md)의 승인이 필요하다. [전체 색인](../INDEX.md).

## END 입력/출력
Raw token+trivia를 정규화하며 세미콜론과 문장을 끝내는 newline을 END로 표현한다. ()/[] 내부 newline은 억제하고 {} 내부에서는 문장 종료를 허용한다. else, dot, comma, 연산자 앞뒤 연속 줄은 억제한다.

## 상세 초안 — D05
끝낼 수 있는 token은 Identifier/Literal/true/false/none/닫는 delimiter/exists 및 bare return/break/continue다. trivia를 건너뛰어 앞뒤 유효 token을 보고 prefix/binary 역할을 구분한다. return 뒤 newline은 bare return 종료를 우선한다. block 닫힘/EOF 직전은 Parser의 terminal boundary로 허용한다.

## 중요 예외
func signature 다음 {, condition 다음 {, else 다음 if/{는 END를 만들지 않는다. 단순 prev/next 표만으로 결정할 수 없는 header는 delimiter/header state로 처리한다. 규칙 표와 예제는 END_RULES를 따른다.

## 검증
세미콜론/줄바꿈의 AST 의미 동등성, operator 양쪽 newline, multi-line call, else 연결, return newline, block comment 내부 newline을 검사한다.
