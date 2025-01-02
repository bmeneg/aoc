(ns aoc.core
  (:gen-class)
  (:require
   [clojure.java.io :as io]
   [clojure.string :as string]))

(defn -remove-one [report]
  (for [i (range 0 (count report))]
	(concat (subvec report 0 i) (subvec report (inc i)))))

(defn -safe? [report]
  (let [pairs (partition 2 1 report)
        diffs (map (fn [[a b]] (- a b)) pairs)]
    (and
     (every? #(and (>= (Math/abs %) 1) (<= (Math/abs %) 3)) diffs)
     (or
      (every? pos? diffs)
      (every? neg? diffs)))))

(defn safe-report? [report]
  (if (-safe? report)
	true
	(some -safe? (-remove-one report))))

(defn -parse-reports [reports]
  (map
   #(mapv read-string (string/split % #"\s+"))
   reports))

(defn -main [& args]
  (let [reports (-parse-reports (line-seq (io/reader *in*)))]
    (do
      (println (count (filter true? (map safe-report? reports))))
      true)))
