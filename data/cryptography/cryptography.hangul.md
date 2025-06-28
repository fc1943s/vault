```쿠즈
```clj
(마프 #(카지 (모드 (+ (* %1 6) 11) 91) 0 1 12 6 (샤르 %)) [10 22 21 10])
(map #(case (mod (+ (* %1 6) 11) 91) 0 1 12 6 (char %)) [10 22 21 10])


(마프 #(샤르 (모드 (+ (* %1 6) 11) 91)) [10 22 21 10])
(map #(char (mod (+ (* %1 6) 11) 91)) [10 22 21 10])
("그" "4" "." "그")
("G" "4" "." "G")


(마프 #(레트 [크 (모드 (+ (* % 6) 35) 91)] (이프 (= 크 35) \# (샤르 크))) [10 22 21 10])
(map #(let [c (mod (+ (* % 6) 35) 91)] (if (= c 35) \# (char c))) [10 22 21 10])


(마프 #(샤르 (모드 (+ (* % 10 10) 10) 10)) [10 10 10 10])
(map #(char (mod (+ (* % 10 10) 10) 10)) [10 10 10 10])


(->> [3 14 0 14] (마프 #(샤르 (모드 (+ (* % 7) 97) 256))))
(->> [3 14 0 14] (map #(char (mod (+ (* % 7) 97) 256))))

(->> [23 1 12 1] (마프 #(샤르 (모드 (+ (* % 19 29) 124) 127))))
(->> [23 1 12 1] (map #(char (mod (+ (* % 19 29) 124) 127))))

(->> [23 1 12 1] (마프 #(샤르 (모드 (+ (* % 19 29) 97) 127))))
(->> [23 1 12 1] (map #(char (mod (+ (* % 19 29) 97) 127))))

(->> [1 12 0 1] (마프 #(샤르 (모드 (+ (* (헤두시 * [1 % 2 3]) % 31) 96) 127))))
(->> [1 12 0 1] (map #(char (mod (+ (* (reduce * [1 % 2 3]) % 31) 96) 127))))


(->> [2 1 13 1] (헤두시 * [1 2 3]) (* 31) (+ 96) (모드 127) (샤르))
(->> [2 1 13 1] (reduce * [1 2 3]) (* 31) (+ 96) (mod 127) (char))



=>(->> [1 1 13 1] (마프 #(샤르 (+ (* % 7) 96))))
=>(->> [1 1 13 1] (map #(char (+ (* % 7) 96))))
("그" "그" "»" "그")
("g" "g" "»" "g")

=>(->> [1 1 13 1] (마프 #(샤르 (+ (* % (헤두시 * (타키 % (이테라치 잉크 2)))) 96))))
=>(->> [1 1 13 1] (map #(char (+ (* % (reduce * (take % (iterate inc 2)))) 96))))
("브" "브" "ࡠ" "브")
("b" "b" "ࡠ" "b")

=>(->> [1 1 13 1] (마프 #(샤르 (+ (* (비트-안드 % 23) (비트-오르 % 23)) 96))))
=>(->> [1 1 13 1] (map #(char (+ (* (bit-and % 23) (bit-or % 23)) 96))))
("" "" "우" "")
("w" "w" "û" "w")

=>(데픙 프 [응] (이프 (<= 응 0) 1 (* 응 (프 (- 응 1))))) (->> [1 1 13 1] (마프 #(샤르 (+ (프 %) 96))))
=>(defn f [n] (if (<= n 0) 1 (* n (f (- n 1))))) (->> [1 1 13 1] (map #(char (+ (f %) 96))))
("아" "아" "챠" "아")
("a" "a" "챠" "a")

=>(레트 [응1 [[1 2] [3 4]] 응2 [[5 6] [7 8]]] (->> [1 1 13 1] (마프 #(샤르 (+ (* % (제트-잉 응1 [0 0])) (제트-잉 응2 [0 1]) 96)))))
=>(let [m1 [[1 2] [3 4]] m2 [[5 6] [7 8]]] (->> [1 1 13 1] (map #(char (+ (* % (get-in m1 [0 0])) (get-in m2 [0 1]) 96)))))
("그" "그" "스" "그")
("g" "g" "s" "g")
```
```

```이스
```exs
[2, 4, 6, 8] |> 이눙.마프(&("#{&1 * 2 - 2}#{&1 * 3 - 9}#{&1 * 5 - 25}#{&1 * 7 - 49}" |> 비치_시지() |> 투_스트링그() <> 투_스트링그(&1 * &1 * 7 + &1 + 95)) |> 스트링그.투_샤를리스트()) |> 이우.인스펙트()
[2, 4, 6, 8] |> Enum.map(&("#{&1 * 2 - 2}#{&1 * 3 - 9}#{&1 * 5 - 25}#{&1 * 7 - 49}" |> byte_size() |> to_string() <> to_string(&1 * &1 * 7 + &1 + 95)) |> String.to_charlist()) |> IO.inspect()


일리시르 -이 "[2, 4, 6, 8] |> 이눙.마프(&(\"#{&1 * 2 - 2}#{&1 * 3 - 9}#{&1 * 5 - 25}#{&1 * 7 - 49}\" |> 비치_시지() |> 투_스트링그() |> (픙 스 -> 인테제르.파르시(투_스트링그(스)) |> 엘렝(0) 인드).() |> (픙 이 -> 투_스트링그(이 * 이 * 7 + 이 + 95) 인드).())) |> 이우.인스펙트()"
elixir -e "[2, 4, 6, 8] |> Enum.map(&(\"#{&1 * 2 - 2}#{&1 * 3 - 9}#{&1 * 5 - 25}#{&1 * 7 - 49}\" |> byte_size() |> to_string() |> (fn x -> Integer.parse(to_string(x)) |> elem(0) end).() |> (fn y -> to_string(y * y * 7 + y + 95) end).())) |> IO.inspect()"


일리시르 -이 '인푸트 = [97, 109, 97]
elixir -e 'input = [97, 109, 97]
코에프스 = 이눙.헤베르시(인푸트)
coeffs = Enum.reverse(input)

위트 아크 <- 1..4
with acc <- 1..4
|> 이눙.마프(픙 스 -> 스 * 스 * 스 * (코에프스 |> 이눙.아트(0)) + 스 * 스 * (코에프스 |> 이눙.아트(1)) + 스 * (코에프스 |> 이눙.아트(2)) + (코에프스 |> 이눙.아트(3)) 인드)
|> Enum.map(fn x -> x * x * x * (coeffs |> Enum.at(0)) + x * x * (coeffs |> Enum.at(1)) + x * (coeffs |> Enum.at(2)) + (coeffs |> Enum.at(3)) end)
|> 이눙.숭(),
|> Enum.sum(),
코데포인트 <- 헹(아크, 1111) + 770,
codepoint <- rem(acc, 1111) + 770,
두: 이우.푸트스(:"\##{코데포인트}")'
do: IO.puts(:"\##{codepoint}")'





일리시르 -이 "이우.푸트스 \"#{이눙.마프([97, 109, 97, 35], 픙 스 -> (스 + 6) * (스 + 4) * (스 + 2) * 스 |> 헹(26) |> 케르네우.+(97) 인드) |> 이눙.마프(&(:이를랑그.인테제르_투_리스트(&1) |> 리스트.피르스트())) |> :우니코지.샤락테르스_투_비나리()}\""
elixir -e "IO.puts \"#{Enum.map([97, 109, 97, 35], fn x -> (x + 6) * (x + 4) * (x + 2) * x |> rem(26) |> Kernel.+(97) end) |> Enum.map(&(:erlang.integer_to_list(&1) |> List.first())) |> :unicode.characters_to_binary()}\""

일리시르 -이 "이우.푸트스 \"#{이눙.마프([97, 109, 97, 35], 픙 스 -> ((스 + 6) * (스 + 4) * (스 + 2) * 스) |> 헹(26) |> 케르네우.+(97) |> :이를랑그.인트_투_샤르 인드) |> 리스트.투_스트링그()}\""
elixir -e "IO.puts \"#{Enum.map([97, 109, 97, 35], fn x -> ((x + 6) * (x + 4) * (x + 2) * x) |> rem(26) |> Kernel.+(97) |> :erlang.int_to_char end) |> List.to_string()}\""



일리시르 -이 "이우.푸트스 \"#{이눙.마프([1, 2, 3], 픙 스 -> (스 + 4) * (스 + 1) * (스 - 1) |> 헹(26) |> 케르네우.-(98)||100 인드 ) |> :이를랑그.인테제르_투_리스트()
elixir -e "IO.puts \"#{Enum.map([1, 2, 3], fn x -> (x + 4) * (x + 1) * (x - 1) |> rem(26) |> Kernel.-(98)||100 end ) |> :erlang.integer_to_list()
# || [109] ++ 이눙.마프([2,3],픙 스-> 97+((2*스-7)*헹(13))||99인드 )
#         || [109] ++ Enum.map([2,3],fn x-> 97+((2*x-7)*rem(13))||99end )
# ||[97]|> 리스트.포우두(&리스트::플라텡/1,[])
#            ||[97]|> List.foldl(&List::flatten/1,[])
# ||:우니코지.샤락테르스_투_비나리()}\""
#            ||:unicode.characters_to_binary()}\""
# 아마
# ama

일리시르 -이 "이우.푸트스 \"#{이눙.마프([1, 2, 3], 픙 스 -> (스 + 4) * (스 + 1) * (스 - 1) |> 헹(26) |> 케르네우.-(98)||100 인드 ) |> 이눙.마프(&(:이를랑그.인테제르_투_리스트(&1) |> 리스트.피르스트())) |> :우니코지.샤락테르스_투_비나리()}\""
elixir -e "IO.puts \"#{Enum.map([1, 2, 3], fn x -> (x + 4) * (x + 1) * (x - 1) |> rem(26) |> Kernel.-(98)||100 end ) |> Enum.map(&(:erlang.integer_to_list(&1) |> List.first())) |> :unicode.characters_to_binary()}\""


일리시르 -이 "이우.푸트스 \"#{이눙.마프([1, 2, 3], 픙 스 -> (스 + 4) * (스 + 1) * (스 - 1) |> 헹(26) |> 케르네우.-(98)||100 인드 ) |> 이눙.마프(&(:이를랑그.인테제르_투_리스트(&1) |> 리스트.피르스트())) |> :우니코지.샤락테르스_투_비나리()}\""
elixir -e "IO.puts \"#{Enum.map([1, 2, 3], fn x -> (x + 4) * (x + 1) * (x - 1) |> rem(26) |> Kernel.-(98)||100 end ) |> Enum.map(&(:erlang.integer_to_list(&1) |> List.first())) |> :unicode.characters_to_binary()}\""







# 잉코지 풍크치옹
# Encode function
플라인테스트 = "엘루"
plaintext = "HELLO"
잉코데드 = 플라인테스트
encoded = plaintext
|> 스트링그.동카지()
|> String.downcase()
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
(스 |> 스트링그.투_샤를리스트() |> 드() |> :이를랑그.비나리_투_인테제르() |> 케르네우.-(97)) * 8 |> 케르네우.+(11)
(x |> String.to_charlist() |> hd() |> :erlang.binary_to_integer() |> Kernel.-(97)) * 8 |> Kernel.+(11)
|> 헹(26)
|> rem(26)
|> 케르네우.+(97)
|> Kernel.+(97)
|> 인테제르.투_샤를리스트()
|> Integer.to_charlist()
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(잉코데드)
IO.puts(encoded)

# 데코지 풍크치옹
# Decode function
시페르테스트 = "프트브크"
ciphertext = "fthvq"
데코데드 = 시페르테스트
decoded = ciphertext
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
(스 |> 스트링그.투_샤를리스트() |> 드() |> :이를랑그.비나리_투_인테제르() |> 케르네우.-(97) |> 케르네우.-(11))
(x |> String.to_charlist() |> hd() |> :erlang.binary_to_integer() |> Kernel.-(97) |> Kernel.-(11))
|> 헹(26 * 8)
|> rem(26 * 8)
|> 헹(26)
|> rem(26)
|> 케르네우.+(97)
|> Kernel.+(97)
|> 인테제르.투_샤를리스트()
|> Integer.to_charlist()
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(데코데드)
IO.puts(decoded)







# 잉코지 풍크치옹
# Encode function
플라인테스트 = "엘루"
plaintext = "HELLO"
잉코데드 = 플라인테스트
encoded = plaintext
|> 스트링그.동카지()
|> String.downcase()
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
(스 |> 스트링그.투_샤를리스트() |> 드() |> :이를랑그.비나리_투_인테제르() |> 케르네우.-(97)) |> 케르네우.*(8) |> 케르네우.+(11)
(x |> String.to_charlist() |> hd() |> :erlang.binary_to_integer() |> Kernel.-(97)) |> Kernel.*(8) |> Kernel.+(11)
|> 헹(26)
|> rem(26)
|> 케르네우.+(97)
|> Kernel.+(97)
|> 인테제르.투_샤를리스트()
|> Integer.to_charlist()
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(잉코데드)
IO.puts(encoded)

# 데코지 풍크치옹
# Decode function
시페르테스트 = "프트브크"
ciphertext = "fthvq"
데코데드 = 시페르테스트
decoded = ciphertext
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
(스 |> 스트링그.투_샤를리스트() |> 드()
(x |> String.to_charlist() |> hd()
|> 케르네우.-(97)
|> Kernel.-(97)
|> 케르네우.-(11)
|> Kernel.-(11)
|> 헹(26 * 8)
|> rem(26 * 8)
|> 헹(26)
|> rem(26)
|> 케르네우.+(97)
|> Kernel.+(97)
|> 인테제르.투_스트링그()
|> Integer.to_string()
|> :이를랑그.비나리_투_인테제르())
|> :erlang.binary_to_integer())
|> 인테제르.투_샤를리스트()
|> Integer.to_charlist()
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(데코데드)
IO.puts(decoded)



일리시
elixi
흐 -이 '# 잉코지 풍크치옹
r -e '# Encode function
플라인테스트 = "엘루"
plaintext = "HELLO"
잉코데드 = 플라인테스트
encoded = plaintext
|> 스트링그.동카지()
|> String.downcase()
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(&(&1 |> 스트링그.투_샤를리스트() |> 드() |> (&1 - 97 - 11) |> 헹(26) |> (&((&1 + 97))) |> 인테제르.투_스트링그() |> <<(&1::우트프8)>>))
|> Enum.map(&(&1 |> String.to_charlist() |> hd() |> (&1 - 97 - 11) |> rem(26) |> (&((&1 + 97))) |> Integer.to_string() |> <<(&1::utf8)>>))
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(잉코데드)
IO.puts(encoded)

# 데코지 풍크치옹
# Decode function
시페르테스트 = "프트브크"
ciphertext = "fthvq"
데코데드 = 시페르테스트
decoded = ciphertext
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(&(&1 |> <<(&1::우트프8)>> |> 스트링그.투_인테제르() |> (&1 - 97 + 11) |> 헹(26) |> (&((&1 + 97))) |> 인테제르.투_스트링그() |> <<(&1::우트프8)>>))
|> Enum.map(&(&1 |> <<(&1::utf8)>> |> String.to_integer() |> (&1 - 97 + 11) |> rem(26) |> (&((&1 + 97))) |> Integer.to_string() |> <<(&1::utf8)>>))
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(데코데드)'
IO.puts(decoded)'
** (콤필레호르) 노필리:6: 네스테드 카프투리스 아리 노트 알로웨드. 요 카노트 데피니 아 풍크치옹 우징그 티 카프투리 오페라토르 & 인시지 아노테르 풍크치옹 데피네드 비아 &. 고트 인발리드 네스테드 카프투리: &(&1 + 97)
** (CompileError) nofile:6: nested captures are not allowed. You cannot define a function using  the capture operator & inside another function defined via &. Got invalid nested capture: &(&1 + 97)
(스트들리브 4.2) 리스트스.이루:1462: :리스트스.마프포우두_1/3
(stdlib 4.2) lists.erl:1462: :lists.mapfoldl_1/3
(스트들리브 4.2) 리스트스.이루:1463: :리스트스.마프포우두_1/3
(stdlib 4.2) lists.erl:1463: :lists.mapfoldl_1/3
(일리시르 1.14.3) 스흐크/일리시르_픙.이루:140: :일리시르_픙.이스카피/3
(elixir 1.14.3) src/elixir_fn.erl:140: :elixir_fn.escape/3
(스트들리브 4.2) 리스트스.이루:1462: :리스트스.마프포우두_1/3
(stdlib 4.2) lists.erl:1462: :lists.mapfoldl_1/3
(일리시르 1.14.3) 스흐크/일리시르_픙.이루:140: :일리시르_픙.이스카피/3
(elixir 1.14.3) src/elixir_fn.erl:140: :elixir_fn.escape/3
(스트들리브 4.2) 리스트스.이루:1462: :리스트스.마프포우두_1/3
(stdlib 4.2) lists.erl:1462: :lists.mapfoldl_1/3
(일리시르 1.14.3) 스흐크/일리시르_픙.이루:140: :일리시르_픙.이스카피/3
(elixir 1.14.3) src/elixir_fn.erl:140: :elixir_fn.escape/3
(일리시르 1.14.3) 이스판징그 마크루: 케르네우.|>/2
(elixir 1.14.3) expanding macro: Kernel.|>/2






# 잉코지 풍크치옹
# Encode function
플라인테스트 = "엘루"
plaintext = "HELLO"
잉코데드 = 플라인테스트
encoded = plaintext
|> 스트링그.동카지()
|> String.downcase()
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
크 = 드(스트링그.투_샤를리스트(스))
c = hd(String.to_charlist(x))
크 = 크 - 97 - 11
c = c - 97 - 11
크 = 헹(크, 26)
c = rem(c, 26)
크 = 크 + 97
c = c + 97
크 = 인테제르.투_스트링그(크)
c = Integer.to_string(c)
<<크::우트프8>>
<<c::utf8>>
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(잉코데드)
IO.puts(encoded)

# 데코지 풍크치옹
# Decode function
시페르테스트 = "프트브크"
ciphertext = "fthvq"
데코데드 = 시페르테스트
decoded = ciphertext
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
크 = 스트링그.투_인테제르(스 |> 텡(픙 스 -> <<스::우트프8>> 인드))
c = String.to_integer(x |> then(fn x -> <<x::utf8>> end))
크 = 크 - 97 + 11
c = c - 97 + 11
크 = 헹(크, 26)
c = rem(c, 26)
크 = 크 + 97
c = c + 97
크 = 인테제르.투_스트링그(크)
c = Integer.to_string(c)
<<크::우트프8>>
<<c::utf8>>
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(데코데드)
IO.puts(decoded)

















# 잉코지 풍크치옹
# Encode function
플라인테스트 = "엘루"
plaintext = "HELLO"
잉코데드 = 플라인테스트
encoded = plaintext
|> 스트링그.동카지()
|> String.downcase()
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
드(스트링그.투_샤를리스트(스)) - 97 - 11
hd(String.to_charlist(x)) - 97 - 11
|> 헹(26)
|> rem(26)
|> & &1 + 97
|> & &1 + 97
|> 인테제르.투_스트링그()
|> Integer.to_string()
|> 리스트.투_스트링그()
|> List.to_string()
|> 스트링그.코데포인트스()
|> String.codepoints()
|> 이눙.피우테르(&스트링그.프린타블리?/1)
|> Enum.filter(&String.printable?/1)
|> 리스트.투_스트링그()
|> List.to_string()
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(잉코데드)
IO.puts(encoded)

# 데코지 풍크치옹
# Decode function
시페르테스트 = "프트브크"
ciphertext = "fthvq"
데코데드 = 시페르테스트
decoded = ciphertext
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
스
x
|> 스트링그.코데포인트스()
|> String.codepoints()
|> 리스트.투_스트링그()
|> List.to_string()
|> 스트링그.투_인테제르()
|> String.to_integer()
|> & &1 - 97 + 11
|> & &1 - 97 + 11
|> 헹(26)
|> rem(26)
|> & &1 + 97
|> & &1 + 97
|> 인테제르.투_스트링그()
|> Integer.to_string()
|> 리스트.투_스트링그()
|> List.to_string()
|> 스트링그.코데포인트스()
|> String.codepoints()
|> 이눙.피우테르(&스트링그.프린타블리?/1)
|> Enum.filter(&String.printable?/1)
|> 리스트.투_스트링그()
|> List.to_string()
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
이우.푸트스(데코데드)
IO.puts(decoded)







플라인테스트 = "엘루"
plaintext = "HELLO"
잉코데드 = 플라인테스트
encoded = plaintext
|> 스트링그.동카지()
|> String.downcase()
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
%{발루이: 샤르} = 리스트.피르스트(스트링그.코데포인트스(스))
%{value: char} = List.first(String.codepoints(x))
샤르 = 샤르 - ?아 + 11 |> 헹(26) |> 케르네우.+(?아)
char = char - ?a + 11 |> rem(26) |> Kernel.+(?a)
인테제르.투_스트링그(샤르)
Integer.to_string(char)
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
|> 스트링그.헤플라시("[]", "")
|> String.replace("[]", "")
|> 이우.푸트스()
|> IO.puts()

시페르테스트 = "프트브크"
ciphertext = "fthvq"
데코데드 = 시페르테스트
decoded = ciphertext
|> 스트링그.그라페메스()
|> String.graphemes()
|> 이눙.마프(픙 스 ->
|> Enum.map(fn x ->
%{발루이: 샤르} = 리스트.피르스트(스트링그.코데포인트스(스))
%{value: char} = List.first(String.codepoints(x))
샤르 = 샤르 - ?아 |> 헹(26) |> 케르네우.+(?아)
char = char - ?a |> rem(26) |> Kernel.+(?a)
인테제르.투_스트링그(샤르)
Integer.to_string(char)
인드)
end)
|> 리스트.투_스트링그()
|> List.to_string()
|> 스트링그.헤플라시("[]", "")
|> String.replace("[]", "")
|> 이우.푸트스()
|> IO.puts()








시프트_폴리 = [1, 0, 1]
shift_poly = [1, 0, 1]

모둘라르_인베르시 = 픙 프 ->
modular_inverse = fn f ->
픙 아, 응 ->
fn a, n ->
카지 헹(응, 아) 두
case rem(n, a) do
0 -> {1, 0, 아}
0 -> {1, 0, a}
브 ->
b ->
{이, 스, 드} = 프.(브, 아)
{y, x, d} = f.(b, a)
{스 - 지브(응, 아) * 이, 이, 드}
{x - div(n, a) * y, y, d}
인드
end
인드
end
인드.(픙 프 -> &(&1.(&1.(&1))) 인드)
end.(fn f -> &(&1.(&1.(&1))) end)

폴리_시프트_샤락테르 = 픙 {샤르, _헤스트}, {아, 브, 크} ->
poly_shift_character = fn {char, _rest}, {a, b, c} ->
시프테드_바우 = 아 * 샤르 * 샤르 + 브 * 샤르 + 크
shifted_val = a * char * char + b * char + c
<<시프테드_바우::우트프8>>
<<shifted_val::utf8>>
인드
end

잉코지 = 픙 프 ->
encode = fn f ->
픙 [샤르 | 타이우], 케이_폴리 ->
fn [char | tail], key_poly ->
[폴리_시프트_샤락테르.({샤르, 0}, 케이_폴리) | 프.(타이우, 케이_폴리)]
[poly_shift_character.({char, 0}, key_poly) | f.(tail, key_poly)]
인드.([], &1)
end.([], &1)
인드.(픙 프 -> &(&1.(&1.(&1))) 인드)
end.(fn f -> &(&1.(&1.(&1))) end)

데코지 = 픙 프 ->
decode = fn f ->
픙 [아, 브 | 타이우], {아_코에프, 브_코에프, 크_코에프} ->
fn [a, b | tail], {a_coeff, b_coeff, c_coeff} ->
인브_아 = 모둘라르_인베르시.(아_코에프, 256) |> 엘렝(0)
inv_a = modular_inverse.(a_coeff, 256) |> elem(0)
_인브_브 = 256 - 브_코에프
_inv_b = 256 - b_coeff
스크르트_테릉 = 혼드(:마트.스크르트(브_코에프 * 브_코에프 - 4 * 아_코에프 * 크_코에프))
sqrt_term = round(:math.sqrt(b_coeff * b_coeff - 4 * a_coeff * c_coeff))
인브_크 = 헹(인브_아 * (브_코에프 * 브_코에프 - 4 * 아_코에프 * 크_코에프), 256)
inv_c = rem(inv_a * (b_coeff * b_coeff - 4 * a_coeff * c_coeff), 256)

샤르_바우 = 헹(인브_아 * (256 + 브_코에프 - 스크르트_테릉), 256)
char_val = rem(inv_a * (256 + b_coeff - sqrt_term), 256)
|> 헹(&(&1 잉 32..126))
|> rem(&(&1 in 32..126))

[<<샤르_바우::우트프8>> | 프.(타이우, {아_코에프, 브_코에프, 크_코에프})]
[<<char_val::utf8>> | f.(tail, {a_coeff, b_coeff, c_coeff})]
인드.([], &1)
end.([], &1)
인드.(픙 프 -> &(&1.(&1.(&1))) 인드)
end.(fn f -> &(&1.(&1.(&1))) end)

플라인테스트 = "엘루 워루드!"
plaintext = "Hello World!"
케이_폴리 = 시프트_폴리
key_poly = shift_poly

시페르테스트 = 잉코지.(스트링그.그라페메스(플라인테스트), 케이_폴리) |> 리스트.투_스트링그()
ciphertext = encode.(String.graphemes(plaintext), key_poly) |> List.to_string()
데코데드 = 데코지.(스트링그.코데포인트스(시페르테스트), 케이_폴리) |> 리스트.투_스트링그()
decoded = decode.(String.codepoints(ciphertext), key_poly) |> List.to_string()

# 오트푸트 티 헤주우트스
# Output the results
이우.푸트스("플라인테스트: #{플라인테스트}")
IO.puts("Plaintext: #{plaintext}")
이우.푸트스("시페르테스트: #{시페르테스트}")
IO.puts("Ciphertext: #{ciphertext}")
이우.푸트스("데코데드: #{데코데드}")
IO.puts("Decoded: #{decoded}")





















데프모둘리 시페르 두
defmodule Cipher do
데프 시프트_샤락테르(<<샤르::우트프8>> = _인푸트, 시프트) 두
def shift_character(<<char::utf8>> = _input, shift) do
시프테드_바우 = 샤르 + 시프트
shifted_val = char + shift
<<시프테드_바우 :: 우트프8>>
<<shifted_val :: utf8>>
인드
end

데프 잉코지(플라인테스트, 케이) 두
def encode(plaintext, key) do
플라인테스트
plaintext
|> 스트링그.코데포인트스()
|> String.codepoints()
|> 이눙.마프(&시프트_샤락테르(&1, 케이))
|> Enum.map(&shift_character(&1, key))
|> 리스트.투_스트링그()
|> List.to_string()
인드
end

데프 데코지(시페르테스트, 케이) 두
def decode(ciphertext, key) do
시페르테스트
ciphertext
|> 스트링그.코데포인트스()
|> String.codepoints()
|> 이눙.마프(&시프트_샤락테르(&1, -케이))
|> Enum.map(&shift_character(&1, -key))
|> 리스트.투_스트링그()
|> List.to_string()
인드
end
인드
end


플라인테스트 = "엘루 워루드!"
plaintext = "Hello World!"
크 = 3
k = 3
시페르테스트 = 시페르.잉코지(플라인테스트, 크)
ciphertext = Cipher.encode(plaintext, k)
데코데드 = 시페르.데코지(시페르테스트, 크)
decoded = Cipher.decode(ciphertext, k)

이우.푸트스("플라인테스트: #{플라인테스트}")
IO.puts("Plaintext: #{plaintext}")
이우.푸트스("시페르테스트: #{시페르테스트}")
IO.puts("Ciphertext: #{ciphertext}")
이우.푸트스("데코데드: #{데코데드}")
IO.puts("Decoded: #{decoded}")










데프모둘리 시프트시페르 두
defmodule ShiftCipher do
데프프 모둘라르_인베르시(_아, 0), 두: {1, 0, 0}
defp modular_inverse(_a, 0), do: {1, 0, 0}
데프프 모둘라르_인베르시(아, 응) 엥 헹(응, 아) != 0 두
defp modular_inverse(a, n) when rem(n, a) != 0 do
{이, 스, 드} = 모둘라르_인베르시(헹(응, 아), 아)
{y, x, d} = modular_inverse(rem(n, a), a)
{스 - 지브(응, 아) * 이, 이, 드}
{x - div(n, a) * y, y, d}
인드
end

데프프 폴리_시프트_샤락테르(샤르, {아, 브, 크}) 두
defp poly_shift_character(char, {a, b, c}) do
아 * 스트링그.투_인테제르(샤르) * 스트링그.투_인테제르(샤르) + 브 * 스트링그.투_인테제르(샤르) + 크 |> 헹(256)
a * String.to_integer(char) * String.to_integer(char) + b * String.to_integer(char) + c |> rem(256)
인드
end

데프 잉코지(플라인테스트, 케이_폴리) 두
def encode(plaintext, key_poly) do
플라인테스트
plaintext
|> 스트링그.코데포인트스()
|> String.codepoints()
|> 이눙.마프(&폴리_시프트_샤락테르(&1, 케이_폴리))
|> Enum.map(&poly_shift_character(&1, key_poly))
인드
end

데프 데코지(시페르테스트, 케이_폴리) 두
def decode(ciphertext, key_poly) do
시페르테스트
ciphertext
|> 스트링그.코데포인트스()
|> String.codepoints()
|> 이눙.슝크_이베리(2, 1, :지스카르드)
|> Enum.chunk_every(2, 1, :discard)
|> 이눙.마프(&데코지_샤락테르(&1, 케이_폴리))
|> Enum.map(&decode_character(&1, key_poly))
|> (픙 스 -> 리스트.투_스트링그(스) 인드).()
|> (fn x -> List.to_string(x) end).()
인드
end

데프프 데코지_샤락테르([아, 브], {아_코에프, 브_코에프, 크_코에프}) 두
defp decode_character([a, b], {a_coeff, b_coeff, c_coeff}) do
인브_아 = 모둘라르_인베르시(아_코에프, 256) |> 엘렝(0)
inv_a = modular_inverse(a_coeff, 256) |> elem(0)
지스크리미난트 = 브_코에프 * 브_코에프 - 4 * 아_코에프 * 크_코에프
discriminant = b_coeff * b_coeff - 4 * a_coeff * c_coeff
카지 지스크리미난트 두
case discriminant do
드 엥 드 < 0 -> 0
d when d < 0 -> 0
_ ->
_ ->
호트1 = (-브_코에프 + :마트.스크르트(지스크리미난트)) * 인브_아 |> 헹(256)
root1 = (-b_coeff + :math.sqrt(discriminant)) * inv_a |> rem(256)
호트2 = (-브_코에프 - :마트.스크르트(지스크리미난트)) * 인브_아 |> 헹(256)
root2 = (-b_coeff - :math.sqrt(discriminant)) * inv_a |> rem(256)
데코데드 =
decoded =
카지 헹(아, 2) 두
case rem(a, 2) do
0 ->
0 ->
이프 헹(아 * 호트1 * 호트1 + 브 * 호트1 + 크_코에프, 256) == 아 두
if rem(a * root1 * root1 + b * root1 + c_coeff, 256) == a do
호트1
root1
에우시
else
호트2
root2
인드
end
_ ->
_ ->
이프 브 >= 0 두
if b >= 0 do
호트1
root1
에우시
else
호트2
root2
인드
end
인드
end
인테제르.투_샤를리스트(데코데드)
Integer.to_charlist(decoded)
인드
end
인드
end
인드
end

플라인테스트 = "엘루 워루드!"
plaintext = "Hello World!"
케이_폴리 = {1, 0, 1}
key_poly = {1, 0, 1}

시페르테스트 = 시프트시페르.잉코지(플라인테스트, 케이_폴리)
ciphertext = ShiftCipher.encode(plaintext, key_poly)
데코데드 = 시프트시페르.데코지(시페르테스트, 케이_폴리)
decoded = ShiftCipher.decode(ciphertext, key_poly)

이우.푸트스("플라인테스트: #{플라인테스트}")
IO.puts("Plaintext: #{plaintext}")
이우.푸트스("시페르테스트: #{시페르테스트}")
IO.puts("Ciphertext: #{ciphertext}")
이우.푸트스("데코데드: #{데코데드}")
IO.puts("Decoded: #{decoded}")


```
```
