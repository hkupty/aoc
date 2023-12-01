(ns hjk.aoc.trebuchet
  (:require [clojure.java.io :as io]
            [clojure.string :as str])
  (:gen-class))

(def digits #{\0 \1 \2 \3 \4 \5 \6 \7 \8 \9})

(defn assoc-word-trie [storage word-chars value]
  (let [[[chr] rst] (split-at 1 word-chars)]
    (cond-> storage
     (seq rst) (update chr assoc-word-trie rst value)
     (empty? rst) (assoc chr {:value value}))))

(defn word-trie [storage ^String word value]
  (assoc-word-trie storage (.toCharArray word) value))

(def digits-trie
  (-> {}
      (word-trie "one" 1)
      (word-trie "two" 2)
      (word-trie "three" 3)
      (word-trie "four" 4)
      (word-trie "five" 5)
      (word-trie "six" 6)
      (word-trie "seven" 7)
      (word-trie "eight" 8)
      (word-trie "nine" 9)
      ))


(def reversed-digits-trie
  (-> {}
      (word-trie "eno" 1)
      (word-trie "owt" 2)
      (word-trie "eerht" 3)
      (word-trie "ruof" 4)
      (word-trie "evif" 5)
      (word-trie "xis" 6)
      (word-trie "neves" 7)
      (word-trie "thgie" 8)
      (word-trie "enin" 9)))

(defn queue
  ([] (clojure.lang.PersistentQueue/EMPTY))
  ([coll]
    (reduce conj clojure.lang.PersistentQueue/EMPTY coll)))

(def not-a-digit (complement digits))

(defn char->int [chr]
  (- (byte chr) 48))

(def take-first-char-as-int (comp (partial drop-while not-a-digit)
                                  first
                                  char->int))

(declare try-from-queue)

(defn try-from-queue [base q]
  (let [q' (cond-> q
             (->> q
                  (isa? clojure.lang.PersistentQueue)
                  (not))
             (queue))
        match (reduce get base q')]
    (cond
      (empty? q') q'
      (:value match) (reduced (:value match))
      (:map? match) q'
      (nil? match) (recur base (pop q'))
      :else q')))

(defn take-char-from-string [base line]
  (reduce
    (fn [acc chr]
      (if (digits chr)
        (reduced (char->int chr))
        (try-from-queue base (conj acc chr))))
    (queue)
    line))

(defn pairs-from-line [^String line]
  (let [chrs (.toCharArray line)]
    [(take-char-from-string digits-trie chrs)
     (take-char-from-string reversed-digits-trie (reverse chrs))]))

(defn process [lines]
  (reduce
    +
    (eduction
      (map pairs-from-line)
      (map (fn [[d1 d2]] (+ d2 (* 10 d1))))
      lines)))

(defn read-input [input]
  (with-open [reader (io/reader (io/resource input))]
    (process (line-seq reader))))

(defn -main [& args]
  (println (read-input "input")))
