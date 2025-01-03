(ns aoc.core-test
  (:require [clojure.test :refer :all]
            [aoc.core :refer :all]))

(def programs
  '("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
	"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
	"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"))
(def operands
  '(["2" "4"] ["5" "5"] ["11" "8"] ["8" "5"]
	["2" "4"] ["5" "5"]	["11" "8"] ["8" "5"]
	["2" "4"] ["8" "5"]))
(def values
  '([2 4] [5 5] [11 8] [8 5]
	[2 4] [5 5] [11 8] [8 5]
	[2 4] [8 5]))

(deftest test--get-operands
  (testing "Get operands from program input"
	(is (= operands (-get-operands programs)))))

(deftest test--get-values
  (testing "Get values from operands"
	(is (= values (-get-values operands)))))

(deftest test--indices-of
  (testing "Get every occurencies of a certain substring in a string"
	(let [expected [1 10 28 37 48 64] ; "mul" indices from the 3rd program
		  result (-indices-of (get (vec programs) 2) "mul")]
	  (is (= expected result)))))

(deftest test--get-branch-indices
  (testing "Get every do and don'ts from a given string as a map collection"
	(let [expected {:do-indices [59] :dont-indices [20]}
		  result (-get-branch-indices (get (vec programs) 2))]
	  (is (= expected result)))))

(deftest test-get-total
  (testing "Get the total sum of multiplications"
	(is (= 370 (get-total values)))))
