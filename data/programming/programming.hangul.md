
## 이스플라잉 티 이-콤비나토르 잉 프트-브르 위트 티 테시니케스 우제드
## explain the y-combinator in pt-br with the techniques used

< 우 이-콤비나토르 에 우마 테크니카 포데로자 다 프로그라마상 푼시오나우, 키 페르미치 아 임플레멘타상 지 헤쿠르상 아노니마 잉 링구아젱스 키 낭 포수엥 수포르치 나치부 파라 에사 푼시오날리다지. 우 콘세이투 두 이-콤비나토르 포이 인트로두지두 포르 아스케우 쿠히 이 헤세베우 에시 노미 잉 헤페렌시아 아우 마테마치쿠 콤비나토르 알론주 슈르시.
< O Y-combinator é uma técnica poderosa da programação funcional, que permite a implementação de recursão anônima em linguagens que não possuem suporte nativo para essa funcionalidade. O conceito do Y-combinator foi introduzido por Haskell Curry e recebeu esse nome em referência ao matemático combinator Alonzo Church.

파라 인텐데르 우 이-콤비나토르, 프리메이루 프레시자무스 인텐데르 아 이데이아 지 헤쿠르상 아노니마. 아 헤쿠르상 아노니마 에 웅 치푸 지 헤쿠르상 잉 키 우마 푼상 시 샤마 아 시 메즈마 셍 헤페렌시아르 세우 프로프리우 노미. 이수 포지 세르 웅 데자피우 잉 링구아젱스 푼시오나이스 키 이지젱 키 아스 푼송이스 세장 노메아다스.
Para entender o Y-combinator, primeiro precisamos entender a ideia de recursão anônima. A recursão anônima é um tipo de recursão em que uma função se chama a si mesma sem referenciar seu próprio nome. Isso pode ser um desafio em linguagens funcionais que exigem que as funções sejam nomeadas.

우 트루키 두 이-콤비나토르 이스타 잉 크리아르 우마 푼상 지 오르뎅 수페리오르 키 아세이타 우마 푼상 이 헤토르나 우마 베르상 헤쿠르시바 데사 푼상, 페르미친두 키 엘라 시 샤미 아 시 메즈마. 에사 푼상 지 오르뎅 수페리오르 에 샤마다 지 콤비나토르 이.
O truque do Y-combinator está em criar uma função de ordem superior que aceita uma função e retorna uma versão recursiva dessa função, permitindo que ela se chame a si mesma. Essa função de ordem superior é chamada de combinator Y.

바무스 이스플리카르 우 이-콤비나토르 파수 아 파수, 우잔두 우마 링구아젱 푼시오나우 이마지나리아 샤마다 르.
Vamos explicar o Y-combinator passo a passo, usando uma linguagem funcional imaginária chamada L.

1. 데피니무스 우마 푼상 아우실리아르 샤마다 이, 키 헤세비 우마 푼상 `프` 코무 아르구멘투: `이 = 풍(프) =>`
1. Definimos uma função auxiliar chamada Y, que recebe uma função `f` como argumento: `Y = fun(f) =>`

2. 덴트루 다 푼상 `이`, 헤토르나무스 우마 노바 푼상 아노니마 키 아세이타 웅 아르구멘투 `스`: `풍(스) =>`
2. Dentro da função `Y`, retornamos uma nova função anônima que aceita um argumento `x`: `fun(x) =>`

3. 아고라, 덴트루 데사 푼상 아노니마, 샤마무스 아 푼상 `프`, 마스 아우 인베스 지 파사르 `스` 지레타멘치, 파사무스 우마 샤마다 헤쿠르시바 파라 `이` 파산두 `스` 코무 아르구멘투: `프(이(프))(스)`
3. Agora, dentro dessa função anônima, chamamos a função `f`, mas ao invés de passar `x` diretamente, passamos uma chamada recursiva para `Y` passando `x` como argumento: `f(Y(f))(x)`

4. 페샤무스 토다스 아스 샤베스 이 파렌테지스: `이 = 풍(프) => (풍(스) => 프(이(프))(스))`
4. Fechamos todas as chaves e parênteses: `Y = fun(f) => (fun(x) => f(Y(f))(x))`

아고라, 포데무스 우자르 우 이-콤비나토르 파라 임플레멘타르 헤쿠르상 아노니마 잉 노사 링구아젱 르. 수포냐 키 데제자무스 카우쿨라르 우 파토리아우 지 웅 누메루 우잔두 헤쿠르상 아노니마. 포데무스 파제르 이수 다 세긴치 마네이라:
Agora, podemos usar o Y-combinator para implementar recursão anônima em nossa linguagem L. Suponha que desejamos calcular o fatorial de um número usando recursão anônima. Podemos fazer isso da seguinte maneira:

1. 데피니무스 우마 푼상 `파토리아우` 키 헤세비 우마 푼상 `프` 이 웅 누메루 `응`: `파토리아우 = 풍(프) => 풍(응) =>`
1. Definimos uma função `fatorial` que recebe uma função `f` e um número `n`: `fatorial = fun(f) => fun(n) =>`

2. 잉 세기다, 베리피카무스 시 `응` 에 이구아우 아 제루. 시 포르, 헤토르나무스 1, 카주 콘트라리우, 샤마무스 아 푼상 `프` 파산두 `응 - 1` 코무 아르구멘투 이 무우치플리카무스 우 헤주우타두 포르 `응`: `이프 (응 == 0) 텡 1 에우시 응 * 프(응 - 1)`
2. Em seguida, verificamos se `n` é igual a zero. Se for, retornamos 1, caso contrário, chamamos a função `f` passando `n - 1` como argumento e multiplicamos o resultado por `n`: `if (n == 0) then 1 else n * f(n - 1)`

3. 페샤무스 토다스 아스 샤베스 이 파렌테지스: `파토리아우 = 풍(프) => 풍(응) => 이프 (응 == 0) 텡 1 에우시 응 * 프(응 - 1)`
3. Fechamos todas as chaves e parênteses: `fatorial = fun(f) => fun(n) => if (n == 0) then 1 else n * f(n - 1)`

아고라, 포데무스 우자르 우 이-콤비나토르 파라 오브테르 우마 베르상 헤쿠르시바 다 푼상 `파토리아우`: `파토리아우헤쿠르시부 = 이(파토리아우)`
Agora, podemos usar o Y-combinator para obter uma versão recursiva da função `fatorial`: `fatorialRecursivo = Y(fatorial)`

아 푼상 `파토리아우헤쿠르시부` 세라 우마 베르상 헤쿠르시바 다 푼상 `파토리아우` 키 포지 세르 샤마다 셍 헤페렌시아르 세우 프로프리우 노미.
A função `fatorialRecursivo` será uma versão recursiva da função `fatorial` que pode ser chamada sem referenciar seu próprio nome.

에사 에 우마 브레비 이스플리카상 두 이-콤비나토르 이 코무 엘리 포지 세르 우자두 파라 임플레멘타르 헤쿠르상 아노니마 잉 링구아젱스 푼시오나이스. 에사 테크니카 에 푼다멘타우 나 프로그라마상 푼시오나우 이 포지 세르 우자다 파라 헤조우베르 프로블레마스 콤플레슈스 키 헤케렝 헤쿠르상.
Essa é uma breve explicação do Y-combinator e como ele pode ser usado para implementar recursão anônima em linguagens funcionais. Essa técnica é fundamental na programação funcional e pode ser usada para resolver problemas complexos que requerem recursão.

---
---
---
---

## 데타이우 잉 프트-브르 10 프로그라밍그 랑구아지 이코지스템스 토다이 잉 티 벤시마르킹그 가미, 브레아킹그 동 헬레반트 카테고리스
## detail in pt-br 10 programming language ecosystems today in the benchmarking game, breaking down relevant categories

1. 피통:
1. Python:

- 아 링구아젱 지 프로그라마상 피통 에 이스트레마멘치 포풀라르 데비두 아 수아 신타시 클라라 이 파시우 지 아프렌데르. 에 암플라멘치 우칠리자두 잉 시엔시아 지 다두스, 아프렌지자두 지 마키나 이 데젠보우비멘투 웨브. 누 조구 지 벤시마르킹그, 피통 텐지 아 테르 웅 데젬페뉴 인페리오르 잉 테르무스 지 벨로시다지 잉 콤파라상 콩 링구아젱스 코무 크 오 자바, 마스 수아 이피시엔시아 지 데젠보우비멘투 제라우멘치 수페라 에사스 데피시엔시아스.
- A linguagem de programação Python é extremamente popular devido à sua sintaxe clara e fácil de aprender. É amplamente utilizado em ciência de dados, aprendizado de máquina e desenvolvimento web. No Jogo de Benchmarking, Python tende a ter um desempenho inferior em termos de velocidade em comparação com linguagens como C ou Java, mas sua eficiência de desenvolvimento geralmente supera essas deficiências.
2. 자바:
2. Java:

- 자바 에 우마 링구아젱 오리엔타다 아 오브제투스 키 에 암플라멘치 우자다 잉 데젠보우비멘투 임프레자리아우 이 아플리카치부스 안드로이드. 텡 웅 봉 데젬페뉴 누 조구 지 벤시마르킹그 데비두 아 수아 이피시엔시아 이 벨로시다지, 이스페시아우멘치 콩 우 우주 다 마키나 비르투아우 자바 (즈븡).
- Java é uma linguagem orientada a objetos que é amplamente usada em desenvolvimento empresarial e aplicativos Android. Tem um bom desempenho no Jogo de Benchmarking devido à sua eficiência e velocidade, especialmente com o uso da Máquina Virtual Java (JVM).
3. 자바스크리프트:
3. JavaScript:

- 자바스크리프트 에 아 링구아젱 지 프로그라마상 프린시파우 파라 데젠보우비멘투 웨브 프론트-인드. 콩 아 인트로두상 두 노지.즈스, 자바스크리프트 탐벵 시 토르노 포풀라르 파라 우 데젠보우비멘투 바크-인드. 누 조구 지 벤시마르킹그, 포지 낭 세르 아 링구아젱 마이스 하피다, 마스 수아 우니베르살리다지 이 베르사칠리다지 토르낭-나 인지스펜사베우.
- JavaScript é a linguagem de programação principal para desenvolvimento web front-end. Com a introdução do Node.js, JavaScript também se tornou popular para o desenvolvimento back-end. No Jogo de Benchmarking, pode não ser a linguagem mais rápida, mas sua universalidade e versatilidade tornam-na indispensável.
4. 크:
4. C:

- 크 에 우마 다스 링구아젱스 지 프로그라마상 마이스 안치가스 이 아인다 에 암플라멘치 우자다 오지, 이스페시아우멘치 잉 시스테마스 오페라시오나이스 이 아르드와리 지 바이슈 니베우. 에 코녜시다 포르 수아 벨로시다지 이 이피시엔시아, 텐두 웅 데젬페뉴 포르치 누 조구 지 벤시마르킹그.
- C é uma das linguagens de programação mais antigas e ainda é amplamente usada hoje, especialmente em sistemas operacionais e hardware de baixo nível. É conhecida por sua velocidade e eficiência, tendo um desempenho forte no Jogo de Benchmarking.
5. 크++:
5. C++:

- 크++ 에 우마 이스텐상 지 크 키 아지시오나 헤쿠르수스 지 프로그라마상 오리엔타다 아 오브제투스. 에 암플라멘치 우자두 잉 소프트와리 지 아우투 데젬페뉴, 코무 조구스. 누 조구 지 벤시마르킹그, 크++ 제라우멘치 텡 웅 데젬페뉴 무이투 봉, 오페레센두 웅 봉 이킬리브리우 엔트리 이피시엔시아 지 코지구 이 벨로시다지 지 이제쿠상.
- C++ é uma extensão de C que adiciona recursos de programação orientada a objetos. É amplamente usado em software de alto desempenho, como jogos. No Jogo de Benchmarking, C++ geralmente tem um desempenho muito bom, oferecendo um bom equilíbrio entre eficiência de código e velocidade de execução.
6. 후비:
6. Ruby:

- 후비 에 우마 링구아젱 지 프로그라마상 지 스크리프트 지 아우투 니베우 코녜시다 포르 수아 신타시 클라라 이 림파. 에 마이스 파모주 펠루 프라메워르크 지 데젠보우비멘투 웨브, 후비 옹 하이우스. 누 조구 지 벤시마르킹그, 우 데젬페뉴 지 후비 포지 낭 세르 우 마이스 포르치, 마스 아 파실리다지 지 우주 이 아 프로두치비다지 두 데젠보우베도르 상 세우스 폰투스 포르치스.
- Ruby é uma linguagem de programação de script de alto nível conhecida por sua sintaxe clara e limpa. É mais famoso pelo framework de desenvolvimento web, Ruby on Rails. No Jogo de Benchmarking, o desempenho de Ruby pode não ser o mais forte, mas a facilidade de uso e a produtividade do desenvolvedor são seus pontos fortes.
7. 스위프트:
7. Swift:

- 스위프트 에 아 링구아젱 지 프로그라마상 프리마리아 파라 데젠보우비멘투 누 이코시스테마 아플리, 잉클루인두 이우스, 마쿠스, 와트슈스 이 트부스. 임보라 세자 우마 링구아젱 마이스 헤센치, 엘라 포이 프로제타다 파라 테르 웅 봉 데젬페뉴, 콩 멜료리아스 노타베이스 소브리 세우 프레데세소르, 오브젝치비-크, 누 조구 지 벤시마르킹그.
- Swift é a linguagem de programação primária para desenvolvimento no ecossistema Apple, incluindo iOS, MacOS, watchOS e tvOS. Embora seja uma linguagem mais recente, ela foi projetada para ter um bom desempenho, com melhorias notáveis sobre seu predecessor, Objective-C, no Jogo de Benchmarking.
8. 구:
8. Go:

- 구 에 우마 링구아젱 지 프로그라마상 데젠보우비다 펠루 고글리 키 시 데스타카 펠라 콩코헨시아 이피시엔치 이 파실리다지 지 우주. 누 조구 지 벤시마르킹그, 구 제라우멘치 텡 웅 봉 데젬페뉴, 콩 웅 템푸 지 이제쿠상 하피두 이 우주 이피시엔치 두스 헤쿠르수스 두 시스테마.
- Go é uma linguagem de programação desenvolvida pelo Google que se destaca pela concorrência eficiente e facilidade de uso. No Jogo de Benchmarking, Go geralmente tem um bom desempenho, com um tempo de execução rápido e uso eficiente dos recursos do sistema.
9. 후스트:
9. Rust:

- 후스트 에 우마 링구아젱 지 시스테마 키 시 데스타카 펠라 세구란사 지 메모리아 이 데젬페뉴. 에 프로제타두 파라 세르 우마 아우테르나치바 마이스 세구라 아 크 이 크++. 누 조구 지 벤시마르킹그, 후스트 노르마우멘치 아프레젠타 데젬페뉴 수페리오르 잉 테르무스 지 벨로시다지 이 이피시엔시아.
- Rust é uma linguagem de sistema que se destaca pela segurança de memória e desempenho. É projetado para ser uma alternativa mais segura a C e C++. No Jogo de Benchmarking, Rust normalmente apresenta desempenho superior em termos de velocidade e eficiência.
10. 코틀링:
10. Kotlin:

- 코틀링 에 우마 링구아젱 지 프로그라마상 모데르나 키 에 토타우멘치 인테로페라베우 콩 자바 이 에 아고라 아 링구아젱 헤코멘다다 파라 우 데젠보우비멘투 지 아플리카치부스 안드로이드. 누 조구 지 벤시마르킹그, 코틀링 텐지 아 테르 웅 데젬페뉴 세멜랸치 아우 자바, 포이스 암부스 푼시오낭 나 즈븡.
- Kotlin é uma linguagem de programação moderna que é totalmente interoperável com Java e é agora a linguagem recomendada para o desenvolvimento de aplicativos Android. No Jogo de Benchmarking, Kotlin tende a ter um desempenho semelhante ao Java, pois ambos funcionam na JVM.

렘브리-시 지 키 우 데젬페뉴 누 조구 지 벤시마르킹그 에 아페나스 우마 다스 무이타스 메트리카스 키 포뎅 세르 우자다스 파라 아발리아르 우마 링구아젱 지 프로그라마상. 아 이스콜랴 다 링구아젱 지 프로그라마상 탐벵 데비 레바르 잉 콘시데라상 오트루스 파토리스, 코무 아 나투레자 두 프로제투, 아 이키피 지 데젠보우비멘투 이 우 이코시스테마 다 링구아젱.
Lembre-se de que o desempenho no Jogo de Benchmarking é apenas uma das muitas métricas que podem ser usadas para avaliar uma linguagem de programação. A escolha da linguagem de programação também deve levar em consideração outros fatores, como a natureza do projeto, a equipe de desenvolvimento e o ecossistema da linguagem.

## 부트 잉 이시 카테고리스 테지 이코지스템스 콤페치 우징그 인트리카치 토우스 잉 티 인치리 한지 오프 아우고리틈스 안드 테시니케스
## but in which categories these ecosystems compete using intricate tools in the entire range of algorithms and techniques

< 우스 이코시스테마스 지 링구아젱스 지 프로그라마상 콤페텡 잉 우마 바리에다지 지 카테고리아스, 잉클루인두, 마스 낭 시 리미탄두 아:
< Os ecossistemas de linguagens de programação competem em uma variedade de categorias, incluindo, mas não se limitando a:

1. 데젬페뉴: 아우구마스 링구아젱스, 코무 크 이 후스트, 상 프로제타다스 파라 테르 아우투 데젬페뉴. 엘라스 상 프레켄테멘치 우자다스 잉 아플리카송이스 온지 아 벨로시다지 에 크루시아우, 코무 시스테마스 오페라시오나이스, 조구스 이 프로세사멘투 지 다두스 잉 그란지 이스칼라.
1. Desempenho: Algumas linguagens, como C e Rust, são projetadas para ter alto desempenho. Elas são frequentemente usadas em aplicações onde a velocidade é crucial, como sistemas operacionais, jogos e processamento de dados em grande escala.

2. 파실리다지 지 우주: 링구아젱스 코무 피통 이 후비 상 파모자스 포르 수아 신타시 클라라 이 인투이치바. 엘라스 상 무이타스 베지스 이스콜리다스 파라 우 프로토치푸 하피두 오 파라 아 인트로두상 지 노바투스 나 프로그라마상.
2. Facilidade de uso: Linguagens como Python e Ruby são famosas por sua sintaxe clara e intuitiva. Elas são muitas vezes escolhidas para o protótipo rápido ou para a introdução de novatos na programação.

3. 세구란사: 링구아젱스 코무 후스트 이 스위프트 포랑 프로제타다스 콩 웅 포르치 인포키 나 세구란사, 잉클루인두 프레벤상 지 에후스 코뭉스, 코무 아세수 아 메모리아 아포스 아 리베라상 (당글링그 포인테르스).
3. Segurança: Linguagens como Rust e Swift foram projetadas com um forte enfoque na segurança, incluindo prevenção de erros comuns, como acesso à memória após a liberação (dangling pointers).

4. 콩코헨시아: 구 이 후스트 상 노타베이스 포르 수아스 푼시오날리다지스 지 콩코헨시아. 아 링구아젱 구 아프레젠타 고로치니스, 잉쿠안투 후스트 오페레시 우마 아보르다젱 지 프로프리에다지 우니카 파라 제렌시아르 우 아세수 아 메모리아 잉 세나리우스 콩코헨치스.
4. Concorrência: Go e Rust são notáveis por suas funcionalidades de concorrência. A linguagem Go apresenta goroutines, enquanto Rust oferece uma abordagem de propriedade única para gerenciar o acesso à memória em cenários concorrentes.

5. 베르사칠리다지: 아우구마스 링구아젱스, 코무 자바스크리프트 이 피통, 상 우자다스 잉 우마 암플라 가마 지 아플리카송이스, 데즈지 우 데젠보우비멘투 웨브 아테 아 시엔시아 지 다두스 이 아프렌지자두 지 마키나.
5. Versatilidade: Algumas linguagens, como JavaScript e Python, são usadas em uma ampla gama de aplicações, desde o desenvolvimento web até a ciência de dados e aprendizado de máquina.

6. 수포르치 아 비블리오테카스 이 프라메워르크스: 아 지스포니빌리다지 지 비블리오테카스 이 프라메워르크스 포지 토르나르 우 데젠보우비멘투 마이스 하피두 이 파시우. 피통 에 파모주 포르 수아 바스타 가마 지 비블리오테카스 파라 시엔시아 지 다두스 이 아프렌지자두 지 마키나, 잉쿠안투 자바스크리프트 포수이 누메로주스 프라메워르크스 파라 우 데젠보우비멘투 웨브.
6. Suporte a bibliotecas e frameworks: A disponibilidade de bibliotecas e frameworks pode tornar o desenvolvimento mais rápido e fácil. Python é famoso por sua vasta gama de bibliotecas para ciência de dados e aprendizado de máquina, enquanto JavaScript possui numerosos frameworks para o desenvolvimento web.

7. 포르타빌리다지: 링구아젱스 코무 자바 이 피통 상 아우타멘치 포르타베이스, 콩 아 카파시다지 지 이제쿠타르 우 메즈무 코지구 잉 지페렌치스 플라타포르마스.
7. Portabilidade: Linguagens como Java e Python são altamente portáveis, com a capacidade de executar o mesmo código em diferentes plataformas.

8. 코무니다지 이 수포르치: 우마 코무니다지 포르치 포지 파제르 우마 그란지 지페렌사 잉 테르무스 지 수포르치 이 지스포니빌리다지 지 헤쿠르수스 지 아프렌지자두. 링구아젱스 코무 자바스크리프트 이 피통 포수엥 그란지스 코무니다지스 지 데젠보우베도리스.
8. Comunidade e suporte: Uma comunidade forte pode fazer uma grande diferença em termos de suporte e disponibilidade de recursos de aprendizado. Linguagens como JavaScript e Python possuem grandes comunidades de desenvolvedores.

9. 인테로페라빌리다지: 링구아젱스 코무 코틀링 이 치페스크리프트 오페레셍 인테로페라빌리다지 콩 자바 이 자바스크리프트, 헤스펙치바멘치, 페르미친두 키 우스 데젠보우베도리스 치렝 프로베이투 다스 카락테리스치카스 데사스 링구아젱스 잉쿠안투 아프로베이탕 우스 베네피시우스 다스 노바스.
9. Interoperabilidade: Linguagens como Kotlin e TypeScript oferecem interoperabilidade com Java e JavaScript, respectivamente, permitindo que os desenvolvedores tirem proveito das características dessas linguagens enquanto aproveitam os benefícios das novas.

10. 오리엔타상 아 오브제투스 브스 푼시오나우: 아우구마스 링구아젱스, 코무 자바 이 크++, 상 포르테멘치 오리엔타다스 아 오브제투스, 잉쿠안투 오트라스, 코무 아스케우, 상 포르테멘치 오리엔타다스 아우 푼시오나우. 아인다 오트라스, 코무 자바스크리프트 이 피통, 수포르탕 암부스 우스 파라지그마스.
10. Orientação a objetos vs funcional: Algumas linguagens, como Java e C++, são fortemente orientadas a objetos, enquanto outras, como Haskell, são fortemente orientadas ao funcional. Ainda outras, como JavaScript e Python, suportam ambos os paradigmas.

## 리스트 10 랑구아지스 헬레아제드 아프테르 2015 데타일링그 인트리카치 데타이우스 잉 오 테이 캉 아펙트 푸투리 벤시마르크스
## list 10 languages released after 2015 detailing intricate details in how they can affect future benchmarks

< 데즈지 2015, 바리아스 노바스 링구아젱스 지 프로그라마상 포랑 란사다스, 카다 우마 트라젠두 헤쿠르수스 이 이노바송이스 우니쿠스 키 포데리앙 임팍타르 푸투루스 벤시마르크스. 아키 이스탕 10 링구아젱스 노타베이스 란사다스 아포스 2015:
< Desde 2015, várias novas linguagens de programação foram lançadas, cada uma trazendo recursos e inovações únicos que poderiam impactar futuros benchmarks. Aqui estão 10 linguagens notáveis lançadas após 2015:

1. 발레리나 (2017):
1. Ballerina (2017):

- 발레리나 에 우마 링구아젱 지 프로그라마상 지 코지구 아베르투 프로제타다 파라 아 에라 다 누벵. 엘라 인트로두스 웅 노부 파라지그마 키 우니 우 코지구 이 아 인프라에스트루투라. 잉 벤시마르크스 푸투루스, 이수 포데리아 아우테라르 아 포르마 코무 메지무스 우 데젬페뉴 두 코지구, 콘시데란두 탐벵 아 콘피구라상 다 인프라에스트루투라.
- Ballerina é uma linguagem de programação de código aberto projetada para a era da nuvem. Ela introduz um novo paradigma que une o código e a infraestrutura. Em benchmarks futuros, isso poderia alterar a forma como medimos o desempenho do código, considerando também a configuração da infraestrutura.
2. 일릉 (2016):
2. Elm (2016):

- 일릉 에 우마 링구아젱 지 프로그라마상 푼시오나우 키 콤필라 파라 자바스크리프트. 에 코녜시다 펠라 수아 포르치 가란치아 지 아우젠시아 지 에후스 지 템푸 지 이제쿠상. 잉 푸투루스 벤시마르크스, 일릉 포지 인플루엔시아르 아 마네이라 코무 메지무스 아 콘피아빌리다지 두 코지구, 알렝 다 페르포르만시.
- Elm é uma linguagem de programação funcional que compila para JavaScript. É conhecida pela sua forte garantia de ausência de erros de tempo de execução. Em futuros benchmarks, Elm pode influenciar a maneira como medimos a confiabilidade do código, além da performance.
3. 지그 (2016):
3. Zig (2016):

- 지그 에 우마 링구아젱 지 프로그라마상 지 우주 제라우 이 지 시스테마 프로제타다 파라 오치미자르 아 마누텐상 지 소프트와리 이 멜료라르 아 프로두치비다지 두 데젠보우베도르. 수아스 카락테리스치카스 지 세구란사 지 메모리아 이 심플리시다지 포뎅 아페타르 푸투루스 벤시마르크스 지 데젬페뉴 이 프로두치비다지.
- Zig é uma linguagem de programação de uso geral e de sistema projetada para otimizar a manutenção de software e melhorar a produtividade do desenvolvedor. Suas características de segurança de memória e simplicidade podem afetar futuros benchmarks de desempenho e produtividade.
4. 줄리아 (2018):
4. Julia (2018):

- 줄리아 에 우마 링구아젱 지 프로그라마상 지 아우투 니베우 이 아우투 데젬페뉴 파라 콤푸타상 테크니카. 텡 아 암비상 지 콤비나르 아 벨로시다지 지 크 콩 아 우자빌리다지 지 피통. 이수 포데리아 무다르 아 포르마 코무 콤파라무스 링구아젱스 지 프로그라마상 잉 테르무스 지 데젬페뉴 이 파실리다지 지 우주 잉 벤시마르크스 푸투루스.
- Julia é uma linguagem de programação de alto nível e alto desempenho para computação técnica. Tem a ambição de combinar a velocidade de C com a usabilidade de Python. Isso poderia mudar a forma como comparamos linguagens de programação em termos de desempenho e facilidade de uso em benchmarks futuros.
5. 헤아존무 (2016):
5. ReasonML (2016):

- 헤아존무 에 우마 노바 신타시 파라 오카무, 크리아다 펠루 파세보크. 에 프로제타다 파라 세르 아세시베우 파라 데젠보우베도리스 자바스크리프트 이 텡 포르치 인테로페라빌리다지 콩 우 자바스크리프트. 이수 포지 인플루엔시아르 푸투루스 벤시마르크스 지 링구아젱스 키 시 인테그랑 아 이코시스테마스 이지스텐치스.
- ReasonML é uma nova sintaxe para OCaml, criada pelo Facebook. É projetada para ser acessível para desenvolvedores JavaScript e tem forte interoperabilidade com o JavaScript. Isso pode influenciar futuros benchmarks de linguagens que se integram a ecossistemas existentes.
6. 크리스타우 (2019):
6. Crystal (2019):

- 크리스타우 에 우마 링구아젱 지 프로그라마상 지 프로포지투 제라우 인스피라다 잉 후비, 마스 콩 데젬페뉴 이 이피시엔시아 세멜랸치스 아 크. 아 콤비나상 지 우마 신타시 아미가베우 콩 데젬페뉴 지 아우투 니베우 포지 임팍타르 푸투루스 벤시마르크스, 이스페시아우멘치 나 콤파라상 엔트리 이피시엔시아 지 데젠보우비멘투 이 데젬페뉴.
- Crystal é uma linguagem de programação de propósito geral inspirada em Ruby, mas com desempenho e eficiência semelhantes a C. A combinação de uma sintaxe amigável com desempenho de alto nível pode impactar futuros benchmarks, especialmente na comparação entre eficiência de desenvolvimento e desempenho.
7. 다르크 (2019):
7. Dark (2019):

- 다르크 에 우마 링구아젱 지 프로그라마상 오리엔타다 아 세르비수스, 프로제타다 파라 토르나르 아 임플란타상 지 코지구 우마 코이자 두 파사두. 세우 임팍투 잉 푸투루스 벤시마르크스 포지 세르 나 포르마 코무 메지무스 아 이피시엔시아 다 임플란타상 이 마누텐상 두 코지구.
- Dark é uma linguagem de programação orientada a serviços, projetada para tornar a implantação de código uma coisa do passado. Seu impacto em futuros benchmarks pode ser na forma como medimos a eficiência da implantação e manutenção do código.
8. 글레앙 (2020):
8. Gleam (2020):

- 글레앙 에 우마 링구아젱 지 프로그라마상 이스타치카 이 포르테멘치 치파다 키 호다 나 이를랑그 븡. 수아스 카락테리스치카스 포데리앙 인플루엔시아르 푸투루스 벤시마르크스, 이스페시아우멘치 누 콘테스투 지 시스테마스 지스트리부이두스 이 콩코헨치스.
- Gleam é uma linguagem de programação estática e fortemente tipada que roda na Erlang VM. Suas características poderiam influenciar futuros benchmarks, especialmente no contexto de sistemas distribuídos e concorrentes.
9. 오징 (2016):
9. Odin (2016):

- 오징 에 우마 링구아젱 지 프로그라마상 지 프로포지투 제라우, 지 아우투 데젬페뉴 이 이스타치카, 프로제타다 콩 우 오브제치부 지 프로두지르 코지구 이피시엔치. 엘라 포지 인플루엔시아르 푸투루스 벤시마르크스 잉 테르무스 지 이피시엔시아 지 데젬페뉴 이 코지구.
- Odin é uma linguagem de programação de propósito geral, de alto desempenho e estática, projetada com o objetivo de produzir código eficiente. Ela pode influenciar futuros benchmarks em termos de eficiência de desempenho e código.
10. 브 (2019):
10. V (2019):

- 브 에 우마 링구아젱 지 프로그라마상 지 시스테마 심플리스 이 하피다, 세멜랸치 아 구. 엘라 프로메치 세구란사, 심플리시다지 이 이피시엔시아, 우 키 포지 아페타르 푸투루스 벤시마르크스 잉 테르무스 지 데젬페뉴 이 세구란사.
- V é uma linguagem de programação de sistema simples e rápida, semelhante a Go. Ela promete segurança, simplicidade e eficiência, o que pode afetar futuros benchmarks em termos de desempenho e segurança.

에사스 링구아젱스 헤프레젠탕 노바스 아보르다젱스 이 필로조피아스 나 프로그라마상. 우 임팍투 델라스 잉 푸투루스 벤시마르크스 데펜데라 지 코무 에사스 필로조피아스 이 카락테리스치카스 세랑 발로리자다스 이 메지다스.
Essas linguagens representam novas abordagens e filosofias na programação. O impacto delas em futuros benchmarks dependerá de como essas filosofias e características serão valorizadas e medidas.

---
---
---
---

## 이프 악치옹스 우제드 토무, 콘베르트 티스 야무 투 토무 투 테스트
## if actions used toml, convert this yaml to toml to test

< 티 야무 콘텐트 프로비데드 헤프레젠트스 아 지투브 악치옹스 워르크플로 필리. 이트 이스 데피네드 잉 아 스페시피크 스트룩투리 이스펙테드 비 지투브.
< The YAML content provided represents a GitHub Actions workflow file. It is defined in a specific structure expected by GitHub.

토무 (통'스 오브비오스, 미니마우 랑구아지) 이스 아 심플리 랑구아지 타트 프로비지스 아 콘피구라치옹 필리 포르마트. 오웨베르, 아스 오프 미 크놀레드지 쿠토프 잉 세프템베르 2021, 지투브 악치옹스 도이스 노트 나치벨리 수포르트 토무 포르 데피닝그 워르크플로스, 엔시 티 콘베르시옹 미그트 노트 비 우제풀 잉 프락치시. 네베르텔레스, 에리'스 오 티 프로비데드 야무 미그트 로크 잉 토무:
TOML (Tom's Obvious, Minimal Language) is a simple language that provides a configuration file format. However, as of my knowledge cutoff in September 2021, GitHub Actions does not natively support TOML for defining workflows, hence the conversion might not be useful in practice. Nevertheless, here's how the provided YAML might look in TOML:

```토무
```toml
[나미]
[name]
발루이 = "헬레아지"
value = "Release"

[옹]
[on]
[옹.푸즈]
[on.push]
브란시스 = ["마잉"]
branches = ["main"]

[조브스]
[jobs]
[조브스.데플로이-위드제트스]
[jobs.deploy-widgets]
훙스-옹 = "우분투-라테스트"
runs-on = "ubuntu-latest"
나미 = "마키 아 풍크치옹 카우 옹 마이네트"
name = "Make a function call on mainnet"

[조브스.데플로이-위드제트스.인브]
[jobs.deploy-widgets.env]
네아르_네트워르크_코넥치옹 = "마이네트"
NEAR_NETWORK_CONNECTION = "mainnet"
네아르_콘트락트_아콘트_이드 = "${{ 바르스.네아르_콘트락트_아콘트_이드 }}"
NEAR_CONTRACT_ACCOUNT_ID = "${{ vars.NEAR_CONTRACT_ACCOUNT_ID }}"
네아르_시그네르_아콘트_이드 = "${{ 바르스.네아르_시그네르_아콘트_이드 }}"
NEAR_SIGNER_ACCOUNT_ID = "${{ vars.NEAR_SIGNER_ACCOUNT_ID }}"
네아르_시그네르_아콘트_푸블리크_케이 = "${{ 바르스.네아르_시그네르_아콘트_푸블리크_케이 }}"
NEAR_SIGNER_ACCOUNT_PUBLIC_KEY = "${{ vars.NEAR_SIGNER_ACCOUNT_PUBLIC_KEY }}"
네아르_시그네르_아콘트_프리바치_케이 = "${{ 세크레트스.네아르_시그네르_아콘트_프리바치_케이 }}"
NEAR_SIGNER_ACCOUNT_PRIVATE_KEY = "${{ secrets.NEAR_SIGNER_ACCOUNT_PRIVATE_KEY }}"

[[조브스.데플로이-위드제트스.스테프스]]
[[jobs.deploy-widgets.steps]]
나미 = "셰코트 헤포지토리"
name = "Checkout repository"
우지스 = "악치옹스/셰코트@브2"
uses = "actions/checkout@v2"

[[조브스.데플로이-위드제트스.스테프스]]
[[jobs.deploy-widgets.steps]]
나미 = "인스타우 네아르 클리"
name = "Install near CLI"
훙 = '''
run = '''
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/near-cli-rs/releases/download/v0.3.1/near-cli-rs-v0.3.1-installer.sh | sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/near-cli-rs/releases/download/v0.3.1/near-cli-rs-v0.3.1-installer.sh | sh
'''
'''

[[조브스.데플로이-위드제트스.스테프스]]
[[jobs.deploy-widgets.steps]]
나미 = "카우 소미 풍크치옹"
name = "Call some function"
훙 = '''
run = '''
네아르 콘트락트 카우-풍크치옹 아스-트란삭치옹 "$네아르_콘트락트_아콘트_이드" '풍크치옹_나미_에리' 즈송-아르그스 '{}' 프레파이드-가스 '100 테라가스' 아타셰드-데포지트 '0 네아르' 시긍-아스 "$네아르_시그네르_아콘트_이드" 네트워르크-콘피그 "$네아르_네트워르크_코넥치옹" 시긍-위트-플라인테스트-프리바치-케이 --시그네르-푸블리크-케이 "$네아르_시그네르_아콘트_푸블리크_케이" --시그네르-프리바치-케이 "$네아르_시그네르_아콘트_프리바치_케이" 센드
near contract call-function as-transaction "$NEAR_CONTRACT_ACCOUNT_ID" 'function_name_here' json-args '{}' prepaid-gas '100 TeraGas' attached-deposit '0 NEAR' sign-as "$NEAR_SIGNER_ACCOUNT_ID" network-config "$NEAR_NETWORK_CONNECTION" sign-with-plaintext-private-key --signer-public-key "$NEAR_SIGNER_ACCOUNT_PUBLIC_KEY" --signer-private-key "$NEAR_SIGNER_ACCOUNT_PRIVATE_KEY" send
'''
'''
```
```

케프 잉 민드 타트 지투브 악치옹스 웡'트 헤코그니지 티스 토무 콘피구라치옹. 이트'스 심플리 아 데몬스트라치옹 오프 오 티 야무 코우드 비 트란즐라테드 투 토무.
Keep in mind that GitHub Actions won't recognize this TOML configuration. It's simply a demonstration of how the YAML could be translated to TOML.

## 오 투 마키 티 콘베르시옹 이벵 모리 콘시지? 콘시데링그 토무'스 이지오마치크 페아투리스
## how to make the conversion even more concise? considering toml's idiomatic features

< 티 토무 포르마트 이스 인데드 데지그네드 투 비 모리 우망-헤아다블리 안드 콘시지, 부트 이트 도에증'트 프로비지 아 와이 투 소르텡 오르 심플리피 티 콘텐트 오프 요르 스페시피크 야무 콘피구라치옹, 베카우지 티 나투리 오프 티 다타 베잉그 헤프레젠테드 이스 스치우 티 사미.
< The TOML format is indeed designed to be more human-readable and concise, but it doesn't provide a way to shorten or simplify the content of your specific YAML configuration, because the nature of the data being represented is still the same.

오웨베르, 요 캉 트리 투 스트룩투리 티 토무 투 헤두시 비주아우 클루테르, 이벵 토그 이트 마이 노트 헤두시 티 리니 콘트 시그니피칸틀리. 에리 이스 아노테르 아템프트, 이시 그로프스 헬라테드 케이스:
However, you can try to structure the TOML to reduce visual clutter, even though it may not reduce the line count significantly. Here is another attempt, which groups related keys:

```토무
```toml
[나미]
[name]
발루이 = "헬레아지"
value = "Release"

[옹.푸즈]
[on.push]
브란시스 = ["마잉"]
branches = ["main"]

[조브스.데플로이-위드제트스]
[jobs.deploy-widgets]
훙스-옹 = "우분투-라테스트"
runs-on = "ubuntu-latest"
나미 = "마키 아 풍크치옹 카우 옹 마이네트"
name = "Make a function call on mainnet"

[조브스.데플로이-위드제트스.인브]
[jobs.deploy-widgets.env]
네아르_네트워르크_코넥치옹 = "마이네트"
NEAR_NETWORK_CONNECTION = "mainnet"
네아르_콘트락트_아콘트_이드 = "${{ 바르스.네아르_콘트락트_아콘트_이드 }}"
NEAR_CONTRACT_ACCOUNT_ID = "${{ vars.NEAR_CONTRACT_ACCOUNT_ID }}"
네아르_시그네르_아콘트_이드 = "${{ 바르스.네아르_시그네르_아콘트_이드 }}"
NEAR_SIGNER_ACCOUNT_ID = "${{ vars.NEAR_SIGNER_ACCOUNT_ID }}"
네아르_시그네르_아콘트_푸블리크_케이 = "${{ 바르스.네아르_시그네르_아콘트_푸블리크_케이 }}"
NEAR_SIGNER_ACCOUNT_PUBLIC_KEY = "${{ vars.NEAR_SIGNER_ACCOUNT_PUBLIC_KEY }}"
네아르_시그네르_아콘트_프리바치_케이 = "${{ 세크레트스.네아르_시그네르_아콘트_프리바치_케이 }}"
NEAR_SIGNER_ACCOUNT_PRIVATE_KEY = "${{ secrets.NEAR_SIGNER_ACCOUNT_PRIVATE_KEY }}"

[조브스.데플로이-위드제트스.스테프스.셰코트]
[jobs.deploy-widgets.steps.checkout]
나미 = "셰코트 헤포지토리"
name = "Checkout repository"
우지스 = "악치옹스/셰코트@브2"
uses = "actions/checkout@v2"

[조브스.데플로이-위드제트스.스테프스.인스타우]
[jobs.deploy-widgets.steps.install]
나미 = "인스타우 네아르 클리"
name = "Install near CLI"
훙 = '''
run = '''
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/near-cli-rs/releases/download/v0.3.1/near-cli-rs-v0.3.1-installer.sh | sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/near-cli-rs/releases/download/v0.3.1/near-cli-rs-v0.3.1-installer.sh | sh
'''
'''

[조브스.데플로이-위드제트스.스테프스.카우]
[jobs.deploy-widgets.steps.call]
나미 = "카우 소미 풍크치옹"
name = "Call some function"
훙 = '''
run = '''
네아르 콘트락트 카우-풍크치옹 아스-트란삭치옹 "$네아르_콘트락트_아콘트_이드" '풍크치옹_나미_에리' 즈송-아르그스 '{}' 프레파이드-가스 '100 테라가스' 아타셰드-데포지트 '0 네아르' 시긍-아스 "$네아르_시그네르_아콘트_이드" 네트워르크-콘피그 "$네아르_네트워르크_코넥치옹" 시긍-위트-플라인테스트-프리바치-케이 --시그네르-푸블리크-케이 "$네아르_시그네르_아콘트_푸블리크_케이" --시그네르-프리바치-케이 "$네아르_시그네르_아콘트_프리바치_케이" 센드
near contract call-function as-transaction "$NEAR_CONTRACT_ACCOUNT_ID" 'function_name_here' json-args '{}' prepaid-gas '100 TeraGas' attached-deposit '0 NEAR' sign-as "$NEAR_SIGNER_ACCOUNT_ID" network-config "$NEAR_NETWORK_CONNECTION" sign-with-plaintext-private-key --signer-public-key "$NEAR_SIGNER_ACCOUNT_PUBLIC_KEY" --signer-private-key "$NEAR_SIGNER_ACCOUNT_PRIVATE_KEY" send
'''
'''
```
```

티스 와이, 이아시 스테프 아스 이트스 옹 우니키 섹치옹, 부트 이트'스 레스 이지오마치크 탕 티 프레비오스 베르시옹 베카우지 토무 아하이스 아리 치피칼리 우제드 투 헤프레젠트 무우치플리 시밀라르 오브젝트스.
This way, each step has its own unique section, but it's less idiomatic than the previous version because TOML arrays are typically used to represent multiple similar objects.

헤멤베르, 지투브 악치옹스 아스 오프 미 크놀레드지 쿠토프 잉 세프템베르 2021, 웡'트 헤코그니지 티스 토무 콘피구라치옹.
Remember, GitHub Actions as of my knowledge cutoff in September 2021, won't recognize this TOML configuration.

## 이프 앙 오피시아우 임플레멘타치옹 웨리 투 비 임플레멘테드, 투릉 티 야무 삼플리 인투 티 모스트 콘시지 토무 이키발렌트
## if an official implementation were to be implemented, turn the yaml sample into the most concise toml equivalent

< 이프 지투브 악치옹스 웨리 투 수포르트 토무 안드 콘시데링그 티 랑구아지'스 페아투리스 포르 콘시제네스, 티 콘피구라치옹 미그트 로크 소메팅그 리키 티스:
< If GitHub Actions were to support TOML and considering the language's features for conciseness, the configuration might look something like this:

```토무
```toml
나미 = "헬레아지"
name = "Release"

[옹.푸즈]
[on.push]
브란시스 = ["마잉"]
branches = ["main"]

[조브스.데플로이-위드제트스]
[jobs.deploy-widgets]
훙스-옹 = "우분투-라테스트"
runs-on = "ubuntu-latest"
나미 = "마키 아 풍크치옹 카우 옹 마이네트"
name = "Make a function call on mainnet"

[조브스.데플로이-위드제트스.인브]
[jobs.deploy-widgets.env]
네아르_네트워르크_코넥치옹 = "마이네트"
NEAR_NETWORK_CONNECTION = "mainnet"
네아르_콘트락트_아콘트_이드 = "${{ 바르스.네아르_콘트락트_아콘트_이드 }}"
NEAR_CONTRACT_ACCOUNT_ID = "${{ vars.NEAR_CONTRACT_ACCOUNT_ID }}"
네아르_시그네르_아콘트_이드 = "${{ 바르스.네아르_시그네르_아콘트_이드 }}"
NEAR_SIGNER_ACCOUNT_ID = "${{ vars.NEAR_SIGNER_ACCOUNT_ID }}"
네아르_시그네르_아콘트_푸블리크_케이 = "${{ 바르스.네아르_시그네르_아콘트_푸블리크_케이 }}"
NEAR_SIGNER_ACCOUNT_PUBLIC_KEY = "${{ vars.NEAR_SIGNER_ACCOUNT_PUBLIC_KEY }}"
네아르_시그네르_아콘트_프리바치_케이 = "${{ 세크레트스.네아르_시그네르_아콘트_프리바치_케이 }}"
NEAR_SIGNER_ACCOUNT_PRIVATE_KEY = "${{ secrets.NEAR_SIGNER_ACCOUNT_PRIVATE_KEY }}"

[[조브스.데플로이-위드제트스.스테프스]]
[[jobs.deploy-widgets.steps]]
나미 = "셰코트 헤포지토리"
name = "Checkout repository"
우지스 = "악치옹스/셰코트@브2"
uses = "actions/checkout@v2"

[[조브스.데플로이-위드제트스.스테프스]]
[[jobs.deploy-widgets.steps]]
나미 = "인스타우 네아르 클리"
name = "Install near CLI"
훙 = '''
run = '''
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/near-cli-rs/releases/download/v0.3.1/near-cli-rs-v0.3.1-installer.sh | sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/near-cli-rs/releases/download/v0.3.1/near-cli-rs-v0.3.1-installer.sh | sh
'''
'''

[[조브스.데플로이-위드제트스.스테프스]]
[[jobs.deploy-widgets.steps]]
나미 = "카우 소미 풍크치옹"
name = "Call some function"
훙 = '''
run = '''
네아르 콘트락트 카우-풍크치옹 아스-트란삭치옹 "$네아르_콘트락트_아콘트_이드" '풍크치옹_나미_에리' 즈송-아르그스 '{}' 프레파이드-가스 '100 테라가스' 아타셰드-데포지트 '0 네아르' 시긍-아스 "$네아르_시그네르_아콘트_이드" 네트워르크-콘피그 "$네아르_네트워르크_코넥치옹" 시긍-위트-플라인테스트-프리바치-케이 --시그네르-푸블리크-케이 "$네아르_시그네르_아콘트_푸블리크_케이" --시그네르-프리바치-케이 "$네아르_시그네르_아콘트_프리바치_케이" 센드
near contract call-function as-transaction "$NEAR_CONTRACT_ACCOUNT_ID" 'function_name_here' json-args '{}' prepaid-gas '100 TeraGas' attached-deposit '0 NEAR' sign-as "$NEAR_SIGNER_ACCOUNT_ID" network-config "$NEAR_NETWORK_CONNECTION" sign-with-plaintext-private-key --signer-public-key "$NEAR_SIGNER_ACCOUNT_PUBLIC_KEY" --signer-private-key "$NEAR_SIGNER_ACCOUNT_PRIVATE_KEY" send
'''
'''
```
```

티스 콘피구라치옹 이스 시밀라르 투 티 프레비오스 이잠플리스, 부트 이트 드로프스 소미 오프 티 네스칭그 (리키 티 '옹' 타블리) 포르 아 모리 플라테네드 안드 콘시지 포르마트. 노치 타트 티스 워우드 온리 비 포시블리 위트 샨지스 투 티 운데를링그 소프트와리 인테르프레칭그 티 콘피구라치옹 필리스, 이시 아스 오프 미 크놀레드지 쿠토프 잉 세프템베르 2021, 도이스 노트 수포르트 토무 잉 지투브 악치옹스.
This configuration is similar to the previous examples, but it drops some of the nesting (like the 'on' table) for a more flattened and concise format. Note that this would only be possible with changes to the underlying software interpreting the configuration files, which as of my knowledge cutoff in September 2021, does not support TOML in GitHub Actions.
