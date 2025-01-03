(ns aoc.core
  (:gen-class)
  (:require [clojure.java.io :as io]))

(defn -parse-programs
  "Get all the mul(?,?) occurencies and their respective numbers."
  [programs]
  (map #(subvec % 1)
	   (mapcat #(re-seq #"mul\((\d{1,3}),(\d{1,3})\)" %) programs)))

(defn -parse-values
  "Get the values from the mul() operation."
  [findings]
  (map #(map read-string %) findings))

(defn get-total
  "Perform the sum of all multiplications."
  [values]
  (reduce + (map #(apply * %) values)))

(defn -main [& args]
  (let [findings (-parse-programs (line-seq (io/reader "input")))]
	(do
	  (println (get-total (-parse-values findings)))
	  true)))
