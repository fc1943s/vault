
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
(let[[y m d][2026 2 25]r 43 e 57400 n 19 D(mod r n)b(+ r D)M(quot r n)N(+(- y r)e D M(.getDayOfYear(java.time.LocalDate/of y m d)))f #(char(nth(mapcat range[b 65 97][(+ b(* M D))(+ b r)(+ 97 D M)])(mod(quot N %)r)))](str(f(* r r))(f r)(f 1)))

;; "W6K"
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
(let [[y1 m1 d1] [2026 1 21] [y2 m2 d2] [2026 4 21] r 43 e 57400 n 19 D (mod r n) b (+ r D) M (quot r n) rr (* r r) A (vec (mapcat range [b 65 97] [(+ b (* M D)) (+ b r) (+ 97 D M)])) c (fn [N p] (char (nth A (mod (quot N p) r)))) s (java.time.LocalDate/of y1 m1 d1) t (java.time.LocalDate/of y2 m2 d2)] (doseq [dt (take-while #(not (.isAfter % t)) (iterate #(.plusDays % 1) s))] (let [y (.getYear dt) N (+ (- y r) e D M (.getDayOfYear dt)) dow (.getDayOfWeek dt) dowS (.getDisplayName dow java.time.format.TextStyle/SHORT java.util.Locale/US)] (printf "%s %s(%d) %c%c%c\n" dt dowS (.getValue dow) (c N rr) (c N r) (c N 1)))) (flush))

;; 2026-01-21 Wed(3) W5S
;; 2026-01-22 Thu(4) W5T
;; 2026-01-23 Fri(5) W5U
;; 2026-01-24 Sat(6) W5V
;; 2026-01-25 Sun(7) W5W
;; 2026-01-26 Mon(1) W5X
;; 2026-01-27 Tue(2) W5Y
;; 2026-01-28 Wed(3) W5Z
;; 2026-01-29 Thu(4) W5a
;; 2026-01-30 Fri(5) W5b
;; 2026-01-31 Sat(6) W5c
;; 2026-02-01 Sun(7) W5d
;; 2026-02-02 Mon(1) W5e
;; 2026-02-03 Tue(2) W5f
;; 2026-02-04 Wed(3) W5g
;; 2026-02-05 Thu(4) W60
;; 2026-02-06 Fri(5) W61
;; 2026-02-07 Sat(6) W62
;; 2026-02-08 Sun(7) W63
;; 2026-02-09 Mon(1) W64
;; 2026-02-10 Tue(2) W65
;; 2026-02-11 Wed(3) W66
;; 2026-02-12 Thu(4) W67
;; 2026-02-13 Fri(5) W68
;; 2026-02-14 Sat(6) W69
;; 2026-02-15 Sun(7) W6A
;; 2026-02-16 Mon(1) W6B
;; 2026-02-17 Tue(2) W6C
;; 2026-02-18 Wed(3) W6D
;; 2026-02-19 Thu(4) W6E
;; 2026-02-20 Fri(5) W6F
;; 2026-02-21 Sat(6) W6G
;; 2026-02-22 Sun(7) W6H
;; 2026-02-23 Mon(1) W6I
;; 2026-02-24 Tue(2) W6J
;; 2026-02-25 Wed(3) W6K
;; 2026-02-26 Thu(4) W6L
;; 2026-02-27 Fri(5) W6M
;; 2026-02-28 Sat(6) W6N
;; 2026-03-01 Sun(7) W6O
;; 2026-03-02 Mon(1) W6P
;; 2026-03-03 Tue(2) W6Q
;; 2026-03-04 Wed(3) W6R
;; 2026-03-05 Thu(4) W6S
;; 2026-03-06 Fri(5) W6T
;; 2026-03-07
;; ... and 1002 more chars
```

---
---
