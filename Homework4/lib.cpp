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

void MergeSort(std::vector<int> & array, int begin, int end) {
  if (begin >= end) { return; }

  int middle = begin + (end - begin) / 2;
  MergeSort(array, begin, middle);
  MergeSort(array, middle + 1, end);
  Merge(array, begin, middle, end);
}

void Merge(std::vector<int> & array, int begin, int middle, int end) {
  int left = middle - begin + 1;
  int right = end - middle;
  std::vector<int> leftArray(left);
  std::vector<int> rightArray(right);

  for (int i = 0; i < left; i++) {
    leftArray[i] = array[left + i];
  }
  for (int i = 0; i < right; i++) {
    rightArray[i] = array[middle + 1 + i];
  }

  int leftArrayIndex = 0, rightArrayIndex = 0;
  int mergedArrayIndex = begin;

  while (leftArrayIndex < left && rightArrayIndex < right) {
    if(leftArray[leftArrayIndex] <= rightArray[rightArrayIndex]) {
      array[mergedArrayIndex] = leftArray[leftArrayIndex];
      leftArrayIndex += 1;
    } else {
      array[mergedArrayIndex] = rightArray[rightArrayIndex];
      rightArrayIndex += 1;
    }
    mergedArrayIndex += 1;
  }

  while (leftArrayIndex < left) {
    array[mergedArrayIndex] = leftArray[leftArrayIndex];
    leftArrayIndex += 1;
    mergedArrayIndex += 1;
  }

  while (rightArrayIndex < right) {
    array[mergedArrayIndex] = rightArray[rightArrayIndex];
    rightArrayIndex += 1;
    mergedArrayIndex += 1;
  }

}

void QuickSort(std::vector<int> &) { std::cout << "QuickSort" << std::endl; }
void RadixSort(std::vector<int> &) { std::cout << "RadixSort" << std::endl; }
void BucketSort(std::vector<int> &) { std::cout << "BucketSort" << std::endl; }
