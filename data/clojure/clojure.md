
---
---

```clojure
(+ 1 2 3)

;; 6
```

```clojure
(let [x 1 y 2 z 3] (+ x y z))

;; 6
```

```clojure
(range 10)

;; (0 1 2 3 4 5 6 7 8 9)
```

```clojure
(let[[y m d][2026 6 14]r 43 e 57400 n 19 D(mod r n)b(+ r D)M(quot r n)N(+(- y r)e D M(.getDayOfYear(java.time.LocalDate/of y m d)))f #(char(nth(mapcat range[b 65 97][(+ b(* M D))(+ b r)(+ 97 D M)])(mod(quot N %)r)))](str(f(* r r))(f r)(f 1)))

;; "W90"
```

```clojure

```

```clojure
(zero? 0)

;; true
```

```clojure
(ns ns1) (defn average [& nums] (/ (reduce + nums) (count nums))) [(average 4 11) (average 3.0 72 9.6 33)]

;; [15/2 29.4]
```

```clojure
(println (((fn [f] ((fn [x] (f (fn [v] ((x x) v)))) (fn [x] (f (fn [v] ((x x) v)))))) (fn [g] (fn [name] (str "Hello, " name "!")))) "John Doe"))

;; Hello, John Doe!
```

```clojure
(require '[clojure.set :as s]) (def a-vowels #{\a \e \i \o \u \x \y \z}) (def b-vowels #{\a \e \i \o \u}) [(s/difference a-vowels b-vowels) (s/union a-vowels b-vowels) (s/intersection a-vowels b-vowels)]

;; [#{\x \y \z} #{\a \e \i \o \u \x \y \z} #{\a \e \i \o \u}]
```

```clojure
(def bhaskara (fn [a b c] (if (or (nil? a) (nil? b) (nil? c)) nil (let [delta (- (* b b) (* 4 a c))] (if (< delta 0) nil (list (/ (+ (- b) (Math/sqrt delta)) (* 2 a)) (/ (- (- b) (Math/sqrt delta)) (* 2 a)))))))) (bhaskara 1 -5 6)

;; (3.0 2.0)
```

```clojure
(defmacro defexpenses [name & expenses] `(def ~name (atom '~expenses))) (defn add-expense [atom-expense amount] (swap! atom-expense conj amount)) (defn sum-expenses [& atoms] (reduce + (map #(apply + @%) atoms))) (defexpenses person-1 1200 800 450) (defexpenses person-2 1000 600 300) (defexpenses person-3 1500 900 550) (add-expense person-1 200) (add-expense person-2 100) (add-expense person-3 150) (sum-expenses person-1 person-2 person-3)

;; 7750
```

```clojure
(defn dot-product [v1 v2] (reduce + (map * v1 v2))) (defn add-elements [v1 v2] (mapv + v1 v2)) (defn apply-weights [input layer-weights layer-biases] (mapv (fn [w b] (+ (dot-product input w) b)) layer-weights layer-biases)) (defn activation-function [input] (mapv #(Math/tanh %) input)) (defn neural-network [input weights biases activation-fn] (let [layer-outputs (map (fn [w b] (activation-fn (apply-weights input w b))) weights biases)] (last layer-outputs))) (def input-1 [0.1 0.2 0.3]) (def input-2 [0.4 0.5 0.6]) (def weights-1 [[0.1 0.2 0.3] [0.4 0.5 0.6] [0.7 0.8 0.9]]) (def biases-1 [0.1 0.2 0.3]) (def weights-2 [[0.1 0.2 0.3] [0.4 0.5 0.6]]) (def biases-2 [0.1 0.2]) (let [inputs [input-1 input-2] weights [weights-1 weights-2] biases [biases-1 biases-2]] (mapv #(neural-network % weights biases activation-function) inputs)) 

;; [[0.23549574953849794 0.47770001216849795] [0.39693043200507755 0.7487042869693086]]
```

```clojure
(def x (-> (promise) (deliver "text")))
@x

;; #'user/x
```

```clojure
(let [[y1 m1 d1] [2026 6 1] [y2 m2 d2] [2026 8 1] r 43 e 57400 n 19 D (mod r n) b (+ r D) M (quot r n) rr (* r r) A (vec (mapcat range [b 65 97] [(+ b (* M D)) (+ b r) (+ 97 D M)])) c (fn [N p] (char (nth A (mod (quot N p) r)))) s (java.time.LocalDate/of y1 m1 d1) t (java.time.LocalDate/of y2 m2 d2)] (doseq [dt (take-while #(not (.isAfter % t)) (iterate #(.plusDays % 1) s))] (let [y (.getYear dt) N (+ (- y r) e D M (.getDayOfYear dt)) dow (.getDayOfWeek dt) dowS (.getDisplayName dow java.time.format.TextStyle/SHORT java.util.Locale/US)] (printf "%s %s(%d) %c%c%c\n" dt dowS (.getValue dow) (c N rr) (c N r) (c N 1)))) (flush))


;; 2026-06-01 Mon(1) W8U
;; 2026-06-02 Tue(2) W8V
;; 2026-06-03 Wed(3) W8W
;; 2026-06-04 Thu(4) W8X
;; 2026-06-05 Fri(5) W8Y
;; 2026-06-06 Sat(6) W8Z
;; 2026-06-07 Sun(7) W8a
;; 2026-06-08 Mon(1) W8b
;; 2026-06-09 Tue(2) W8c
;; 2026-06-10 Wed(3) W8d
;; 2026-06-11 Thu(4) W8e
;; 2026-06-12 Fri(5) W8f
;; 2026-06-13 Sat(6) W8g
;; 2026-06-14 Sun(7) W90
;; 2026-06-15 Mon(1) W91
;; 2026-06-16 Tue(2) W92
;; 2026-06-17 Wed(3) W93
;; 2026-06-18 Thu(4) W94
;; 2026-06-19 Fri(5) W95
;; 2026-06-20 Sat(6) W96
;; 2026-06-21 Sun(7) W97
;; 2026-06-22 Mon(1) W98
;; 2026-06-23 Tue(2) W99
;; 2026-06-24 Wed(3) W9A
;; 2026-06-25 Thu(4) W9B
;; 2026-06-26 Fri(5) W9C
;; 2026-06-27 Sat(6) W9D
;; 2026-06-28 Sun(7) W9E
;; 2026-06-29 Mon(1) W9F
;; 2026-06-30 Tue(2) W9G
;; 2026-07-01 Wed(3) W9H
;; 2026-07-02 Thu(4) W9I
;; 2026-07-03 Fri(5) W9J
;; 2026-07-04 Sat(6) W9K
;; 2026-07-05 Sun(7) W9L
;; 2026-07-06 Mon(1) W9M
;; 2026-07-07 Tue(2) W9N
;; 2026-07-08 Wed(3) W9O
;; 2026-07-09 Thu(4) W9P
;; 2026-07-10 Fri(5) W9Q
;; 2026-07-11 Sat(6) W9R
;; 2026-07-12 Sun(7) W9S
;; 2026-07-13 Mon(1) W9T
;; 2026-07-14 Tue(2) W9U
;; 2026-07-15 Wed(3) W9V
;; 2026-07-16
;; ... and 364 more chars
```

---
---
