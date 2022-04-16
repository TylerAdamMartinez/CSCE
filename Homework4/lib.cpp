#include <iostream>
#include <vector>

// custom header file for sorting funcs
#include "lib.h"

void vec_swap(std::vector<int> & array, int a, int b) {
  int temp = array.at(a);
  array.at(a) = array.at(b);
  array.at(b) = temp;
}

void InsertionSort(std::vector<int> & array) {
  int current_pos, key, prev_pos;

  for( current_pos = 1; current_pos != array.size(); current_pos++ ) {
    key = array.at(current_pos);
    prev_pos = current_pos - 1;

    while(prev_pos >= 0 && array.at(prev_pos) > key) {
      array.at(prev_pos + 1) = array.at(prev_pos);
      prev_pos--;
    }

    array.at(prev_pos + 1) = key;
  }
}

void SelectionSort(std::vector<int> &) { std::cout << "SelectionSort" << std::endl; }
void HeapSort(std::vector<int> &) { std::cout << "HeapSort" << std::endl; }
void MergeSort(std::vector<int> &) { std::cout << "MergeSort" << std::endl; }
void QuickSort(std::vector<int> &) { std::cout << "QuickSort" << std::endl; }
void RadixSort(std::vector<int> &) { std::cout << "RadixSort" << std::endl; }
void BucketSort(std::vector<int> &) { std::cout << "BucketSort" << std::endl; }
