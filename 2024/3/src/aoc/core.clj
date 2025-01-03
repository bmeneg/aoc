(ns aoc.core
  (:gen-class)
  (:require
   [clojure.string :as string]
   [clojure.java.io :as io]))

(defn -get-operands
  "Get both operands from every mul(?,?) operation."
  [programs]
  (map #(subvec % 1)
	   (mapcat #(re-seq #"mul\((\d{1,3}),(\d{1,3})\)" %) programs)))

(defn -get-values
  "Get the real integer values from the string operands."
  [operands]
  (map #(map read-string %) operands))

(defn -indices-of
  "Get all occurencies of a certain substring in a string."
  [s substr]
  (loop [idx 0
		 indices []]
	(let [found-idx (string/index-of s substr idx)]
	  (if found-idx
		(recur (inc found-idx) (conj indices found-idx))
		indices))))

(defn -get-branch-indices
  "Get the index for every do/don't in the program."
  [program]
  {:do-indices (-indices-of program "do()")
   :dont-indices (-indices-of program "don't()")})

(defn get-total
  "Perform the sum of all multiplications."
  [values]
  (reduce + (map #(apply * %) values)))

(defn -main [& args]
  (let [programs (line-seq (io/reader "input"))]
	(do
	  (println (get-total (-get-values (-get-operands programs))))
	  true)))
