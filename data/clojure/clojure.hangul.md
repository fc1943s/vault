
---
---
---
---

```클로주리
```clojure
(+ 1 2 3)
(+ 1 2 3)

 6
;; 6
```
```

```클로주리
```clojure
(레트 [스 1 이 2 스 3] (+ 스 이 스))
(let [x 1 y 2 z 3] (+ x y z))

 6
;; 6
```
```

```클로주리
```clojure
(한지 10)
(range 10)

 (0 1 2 3 4 5 6 7 8 9)
;; (0 1 2 3 4 5 6 7 8 9)
```
```

```클로주리
```clojure
(제루? 0)
(zero? 0)

 트루이
;; true
```
```

```클로주리
```clojure
(응스 응스1) (데픙 아베라지 [& 눔스] (/ (헤두시 + 눔스) (콘트 눔스))) [(아베라지 4 11) (아베라지 3.0 72 9.6 33)]
(ns ns1) (defn average [& nums] (/ (reduce + nums) (count nums))) [(average 4 11) (average 3.0 72 9.6 33)]

 [15/2 29.4]
;; [15/2 29.4]
```
```

```클로주리
```clojure
(프린틀릉 (((픙 [프] ((픙 [스] (프 (픙 [브] ((스 스) 브)))) (픙 [스] (프 (픙 [브] ((스 스) 브)))))) (픙 [그] (픙 [나미] (스트르 "엘루, " 나미 "!")))) "종 도이"))
(println (((fn [f] ((fn [x] (f (fn [v] ((x x) v)))) (fn [x] (f (fn [v] ((x x) v)))))) (fn [g] (fn [name] (str "Hello, " name "!")))) "John Doe"))

 엘루, 종 도이!
;; Hello, John Doe!
```
```

```클로주리
```clojure
(헤키리 '[클로주리.세트 :아스 스]) (데프 아-보웨우스 #{\아 \이 \이 \우 \우 \스 \이 \스}) (데프 브-보웨우스 #{\아 \이 \이 \우 \우}) [(스/지페렌시 아-보웨우스 브-보웨우스) (스/우니옹 아-보웨우스 브-보웨우스) (스/인테르섹치옹 아-보웨우스 브-보웨우스)]
(require '[clojure.set :as s]) (def a-vowels #{\a \e \i \o \u \x \y \z}) (def b-vowels #{\a \e \i \o \u}) [(s/difference a-vowels b-vowels) (s/union a-vowels b-vowels) (s/intersection a-vowels b-vowels)]

 [#{\스 \이 \스} #{\아 \이 \이 \우 \우 \스 \이 \스} #{\아 \이 \이 \우 \우}]
;; [#{\x \y \z} #{\a \e \i \o \u \x \y \z} #{\a \e \i \o \u}]
```
```

```클로주리
```clojure
(데프 바스카라 (픙 [아 브 크] (이프 (오르 (니우? 아) (니우? 브) (니우? 크)) 니우 (레트 [데우타 (- (* 브 브) (* 4 아 크))] (이프 (< 데우타 0) 니우 (리스트 (/ (+ (- 브) (마트/스크르트 데우타)) (* 2 아)) (/ (- (- 브) (마트/스크르트 데우타)) (* 2 아)))))))) (바스카라 1 -5 6)
(def bhaskara (fn [a b c] (if (or (nil? a) (nil? b) (nil? c)) nil (let [delta (- (* b b) (* 4 a c))] (if (< delta 0) nil (list (/ (+ (- b) (Math/sqrt delta)) (* 2 a)) (/ (- (- b) (Math/sqrt delta)) (* 2 a)))))))) (bhaskara 1 -5 6)

 (3.0 2.0)
;; (3.0 2.0)
```
```

```클로주리
```clojure
(데프마크루 데페스펜시스 [나미 & 이스펜시스] `(데프 응나미 (아통 '응이스펜시스))) (데픙 아드-이스펜시 [아통-이스펜시 아몬트] (스와프! 아통-이스펜시 콘즈 아몬트)) (데픙 숭-이스펜시스 [& 아톰스] (헤두시 + (마프 #(아플리 + @%) 아톰스))) (데페스펜시스 페르송-1 1200 800 450) (데페스펜시스 페르송-2 1000 600 300) (데페스펜시스 페르송-3 1500 900 550) (아드-이스펜시 페르송-1 200) (아드-이스펜시 페르송-2 100) (아드-이스펜시 페르송-3 150) (숭-이스펜시스 페르송-1 페르송-2 페르송-3)
(defmacro defexpenses [name & expenses] `(def ~name (atom '~expenses))) (defn add-expense [atom-expense amount] (swap! atom-expense conj amount)) (defn sum-expenses [& atoms] (reduce + (map #(apply + @%) atoms))) (defexpenses person-1 1200 800 450) (defexpenses person-2 1000 600 300) (defexpenses person-3 1500 900 550) (add-expense person-1 200) (add-expense person-2 100) (add-expense person-3 150) (sum-expenses person-1 person-2 person-3)

 7750
;; 7750
```
```

```클로주리
```clojure
(데픙 도트-프로둑트 [브1 브2] (헤두시 + (마프 * 브1 브2))) (데픙 아드-일레멘트스 [브1 브2] (마프브 + 브1 브2)) (데픙 아플리-웨이그트스 [인푸트 라예르-웨이그트스 라예르-비아지스] (마프브 (픙 [ 브] (+ (도트-프로둑트 인푸트 ) 브)) 라예르-웨이그트스 라예르-비아지스)) (데픙 악치바치옹-풍크치옹 [인푸트] (마프브 #(마트/탄 %) 인푸트)) (데픙 네우라우-네트워르크 [인푸트 웨이그트스 비아지스 악치바치옹-픙] (레트 [라예르-오트푸트스 (마프 (픙 [ 브] (악치바치옹-픙 (아플리-웨이그트스 인푸트 브))) 웨이그트스 비아지스)] (라스트 라예르-오트푸트스))) (데프 인푸트-1 [0.1 0.2 0.3]) (데프 인푸트-2 [0.4 0.5 0.6]) (데프 웨이그트스-1 [[0.1 0.2 0.3] [0.4 0.5 0.6] [0.7 0.8 0.9]]) (데프 비아지스-1 [0.1 0.2 0.3]) (데프 웨이그트스-2 [[0.1 0.2 0.3] [0.4 0.5 0.6]]) (데프 비아지스-2 [0.1 0.2]) (레트 [인푸트스 [인푸트-1 인푸트-2] 웨이그트스 [웨이그트스-1 웨이그트스-2] 비아지스 [비아지스-1 비아지스-2]] (마프브 #(네우라우-네트워르크 % 웨이그트스 비아지스 악치바치옹-풍크치옹) 인푸트스))
(defn dot-product [v1 v2] (reduce + (map * v1 v2))) (defn add-elements [v1 v2] (mapv + v1 v2)) (defn apply-weights [input layer-weights layer-biases] (mapv (fn [w b] (+ (dot-product input w) b)) layer-weights layer-biases)) (defn activation-function [input] (mapv #(Math/tanh %) input)) (defn neural-network [input weights biases activation-fn] (let [layer-outputs (map (fn [w b] (activation-fn (apply-weights input w b))) weights biases)] (last layer-outputs))) (def input-1 [0.1 0.2 0.3]) (def input-2 [0.4 0.5 0.6]) (def weights-1 [[0.1 0.2 0.3] [0.4 0.5 0.6] [0.7 0.8 0.9]]) (def biases-1 [0.1 0.2 0.3]) (def weights-2 [[0.1 0.2 0.3] [0.4 0.5 0.6]]) (def biases-2 [0.1 0.2]) (let [inputs [input-1 input-2] weights [weights-1 weights-2] biases [biases-1 biases-2]] (mapv #(neural-network % weights biases activation-function) inputs))

 [[0.23549574953849794 0.47770001216849795] [0.39693043200507755 0.7487042869693086]]
;; [[0.23549574953849794 0.47770001216849795] [0.39693043200507755 0.7487042869693086]]
```
```

```클로주리
```clojure
(데프 스 (-> (프로미지) (델리베르 "테스트")))
(def x (-> (promise) (deliver "text")))
@스
@x

 #'우제르/스
;; #'user/x
```
```

---
---
---
---
