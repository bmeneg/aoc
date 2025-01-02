(ns aoc.core-test
  (:require [clojure.test :refer :all]
			[clojure.string :as str]
            [aoc.core :refer :all]))

(def input-data
  '("7 6 4 2 1"
	"1 2 7 8 9"
	"9 7 6 2 1"
	"1 3 2 4 5"
	"8 6 4 4 1"
	"1 3 6 7 9"))

(deftest test--parse-reports
  (testing "Parsing reports"
	(is (= (-parse-reports input-data)
		   '([7 6 4 2 1]
			 [1 2 7 8 9]
			 [9 7 6 2 1]
			 [1 3 2 4 5]
			 [8 6 4 4 1]
			 [1 3 6 7 9])))))

(deftest test-safe-report?
  (testing "Check for safe reports"
	(is (safe-report? [7 6 4 2 1]))
	(is (safe-report? [1 3 2 4 5]))
	(is (safe-report? [8 6 4 4 1]))
	(is (safe-report? [1 3 6 7 9]))
	(is (not (safe-report? [1 2 7 8 9])))
	(is (not (safe-report? [9 7 6 2 1])))
	(is (not (safe-report? [1 5 9 10 11])))))
