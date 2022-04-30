#include <iostream>
#include <vector>
#include <algorithm>
#include <time.h>
// custom header files for sorting & unit tests funcs
#include "lib.h"
#include "tests.h"
// testing purposes
#include <cassert>
#define assertm(exp, msg) assert(((void)msg, exp))

int main() {
  std::cout << "homework 4 unit tests" << std::endl;

  InsertionSortTest();
  SelectionSortTest();
  QuickSortTest();
  MergeSortTest();
  HeapSortTest();
  RadixSortTest();
  BucketSortTest();
  return 0;
}

void rand_populate_vector(std::vector<int> & array, int size) {
  srand(time(NULL));
  for (int i = 0; i < size; i++) {
    array.push_back(rand() % 100 + 1);
  }
}

std::vector<int> sorted_vector(std::vector<int> array) {
  std::sort(array.begin(), array.end());
  return array;
}

void assert_eq(std::vector<int> & array0, std::vector<int> & array1, std::string testName) {
  if(array0 == array1) { 
    std::cout << testName << " [passed]" << std::endl;
  }
  else {
    std::cout << testName <<  " [failed]" << std::endl;

    std::cout << "Array0" << std::endl;
    for(std::vector<int>::iterator it = array0.begin(); it != array0.end(); it++) {
      std::cout << *it << ", ";
    }
    std::cout << std::endl;

    std::cout << "Array1" << std::endl;
    for(std::vector<int>::iterator it = array1.begin(); it != array1.end(); it++) {
      std::cout << *it << ", ";
    }
    std::cout << std::endl;
  }
}

void InsertionSortTest() {
  std::vector<int> array;
  rand_populate_vector(array, 15);
  std::vector<int> control_array = sorted_vector(array);

  InsertionSort(array);
  assert_eq(control_array, array, "InsertionSortTest");
}

void SelectionSortTest() {
  std::vector<int> array;
  rand_populate_vector(array, 10);
  std::vector<int> control_array = sorted_vector(array);

  SelectionSort(array);
  assert_eq(control_array, array, "SelectionSortTest");

}
void HeapSortTest() {
  std::vector<int> array;
  rand_populate_vector(array, 5);
  std::vector<int> control_array = sorted_vector(array);

  HeapSort(array, array.size());
  assert_eq(control_array, array, "HeapSortTest");
}

void MergeSortTest() {
  std::vector<int> array;
  rand_populate_vector(array, 8);
  std::vector<int> control_array = sorted_vector(array);

  MergeSort(array, 0, array.size() - 1);
  assert_eq(control_array, array, "MergeSortTest");
}

void QuickSortTest() {
  std::vector<int> array;
  rand_populate_vector(array, 8);
  std::vector<int> control_array = sorted_vector(array);

  QuickSort(array, 0, array.size() - 1);
  assert_eq(control_array, array, "QuickSortTest");
}
void RadixSortTest() {}
void BucketSortTest() {}
