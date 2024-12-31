(ns aoc.core
  (:gen-class)
  (:require
   [clojure.java.io :as io]
   [clojure.string :as string]))

(defn not-empty? [coll]
  (not (empty? coll)))

(defn get-lists [content]
  (let [pairs (map #(string/split % #"\s+") content)]
	(let [first-list (mapv #(read-string (first %)) pairs)
		  second-list (mapv #(read-string (second %)) pairs)]
	  {:first-list (sort first-list)
	   :second-list (sort second-list)})))

(defn calc-lists-distance [lists]
  (let [first-list (:first-list lists)
		second-list (:second-list lists)]
	(reduce +
			(map #(abs (- %1 %2)) first-list second-list))))

(defn calc-lists-similarity [lists]
  (let [first-list (:first-list lists)
		second-list (:second-list lists)]
	(reduce +
			(map (fn [n]
				   (* n (count (filter #(= n %) second-list))))
				 (filter #(some #{%} second-list) first-list)))))

(defn -main [& args]
  (let [lists (get-lists (line-seq (io/reader *in*)))
		distance (calc-lists-distance lists)
		similarity-score (calc-lists-similarity lists)]
	(do
	  (println distance)
	  (println similarity-score)
	  true)))
