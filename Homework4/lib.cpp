#include <iostream>
#include <vector>

// custom header file for sorting funcs
#include "lib.h"

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

void SelectionSort(std::vector<int> & array) {
  int min_index, temp;

  for (int i = 0; i < array.size() - 1; i++) {
    min_index = i;
    for (int j = i + 1; j < array.size(); j++) {
      if (array.at(j) < array.at(min_index)) { min_index = j; }
    }
    temp = array.at(min_index);
    array.at(min_index) = array.at(i);
    array.at(i) = temp;
  }
}

void HeapSort(std::vector<int> & array, int heapSize) {
  for (int i = heapSize / 2 - 1; i >=0; i--) {
    Heap(array, heapSize, i);
  }

  int temp;
  for (int i = heapSize - 1; i > 0; i--) {
    temp = array.at(0);
    array.at(0) = array.at(i);
    array.at(i) = temp;

    Heap(array, i, 0);
  }
}

void Heap(std::vector<int> & array, int heapSize, int index) {
  int largest_num = index;
  int left = 2 * index + 1;
  int right = 2 * index + 2;

  if (left < heapSize && array.at(left) > array.at(largest_num)) {
    largest_num = left;
  }
  if (right < heapSize && array.at(right) > array.at(largest_num)) {
    largest_num = right;
  }
  int temp;
  if (largest_num != index) {
    temp = array.at(index);
    array.at(index) = array.at(largest_num);
    array.at(largest_num) = temp;
  }
}

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
  std::vector<int> leftArray;
  std::vector<int> rightArray;

  for (int i = 0; i < left; i++) {
    leftArray.push_back(array.at(left + i));
  }
  for (int i = 0; i < right; i++) {
    rightArray.push_back(array.at(middle + 1 + i));
  }

  int leftArrayIndex = 0;
  int rightArrayIndex = 0;
  int mergedArrayIndex = begin;

  while (leftArrayIndex < left && rightArrayIndex < right) {
    if(leftArray.at(leftArrayIndex) <= rightArray.at(rightArrayIndex)) {
      array.at(mergedArrayIndex) = leftArray.at(leftArrayIndex);
      leftArrayIndex++;
    } else {
      array.at(mergedArrayIndex) = rightArray.at(rightArrayIndex);
      rightArrayIndex++;
    }
    mergedArrayIndex++;
  }

  while (leftArrayIndex < left) {
    array.at(mergedArrayIndex) = leftArray.at(leftArrayIndex);
    leftArrayIndex++;
    mergedArrayIndex++;
  }

  while (rightArrayIndex < right) {
    array.at(mergedArrayIndex) = rightArray.at(rightArrayIndex);
    rightArrayIndex++;
    mergedArrayIndex++;
  }

}

void QuickSort(std::vector<int> & array, int low, int high) {
  if (high > low) {
    int partition_index = QuickSortPartition(array, low, high);
    QuickSort(array, low, partition_index - 1);
    QuickSort(array, partition_index + 1, high);
  }
}

int QuickSortPartition(std::vector<int> & array, int low, int high) {
  int pivot = array.at(high);
  int index = low - 1;
  int temp;

  for (int i = low; i < high; i++) {
    if (array.at(i) < pivot) {
      index++;
      temp = array.at(index);
      array.at(index) = array.at(i);
      array.at(i) = temp;
    }
  }

  temp = array.at(index + 1);
  array.at(index + 1) = array.at(high);
  array.at(high) = temp;
  return index + 1;
}

void RadixSort(std::vector<int> & array) {
  int max = array.at(0);
  const int d = 1000;

  for (int i = 1; i < array.size(); i++) {
    if (array.at(i) > max) { max = array.at(i); }
  }

  for (int i = 1; max / i > 0; i *= d) {
    RadixSortCount(array, i);
  }
}

void RadixSortCount(std::vector<int> & array, int index) {
  const int d = 1000;
  std::vector <int> Output(array.size());
  std::vector <int> Count(d);

  for (int i = 0; i < array.size(); i++) {
    Count.at((array.at(i) / index) % d)++;
  }

  for (int i = 1; i < 10; i++) {
    Output.at(Count.at((array.at(i) / index) % d) - 1) = array.at(i);
    Count.at((array.at(i) / index) % d)--;
  }

  for (int i = 0; i < array.size(); i++) {
    array.at(i) = Output.at(i);
  }
}

void BucketSort(std::vector<int> &) { std::cout << "BucketSort" << std::endl; }
