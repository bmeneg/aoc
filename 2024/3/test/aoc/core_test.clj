(ns aoc.core-test
  (:require [clojure.test :refer :all]
            [aoc.core :refer :all]))

(def programs
  '("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
	"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
	"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"))
(def findings
  '(["2" "4"] ["5" "5"] ["11" "8"] ["8" "5"]
	["2" "4"] ["5" "5"]	["11" "8"] ["8" "5"]
	["2" "4"] ["8" "5"]))
(def values
  '([2 4] [5 5] [11 8] [8 5]
	[2 4] [5 5] [11 8] [8 5]
	[2 4] [8 5]))

(deftest test--parse-programs
  (testing "Parse program input"
	(is (-parse-programs programs) findings)))

(deftest test--parse-values
  (testing "Parse values from program parse findings"
	(is (-parse-values findings) values)))

(deftest test-get-total
  (testing "Get the total sum of multiplications"
	(is (get-total values) 322)))
