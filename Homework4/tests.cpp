#include <iostream>
#include <vector>
#include <algorithm>
#include <time.h>
// custom header files for sorting & unit tests funcs
#include "lib.h"
#include "tests.h"
// define consts
#define hundread_thousand 100000
#define one_million 1000000

int main() {
  std::cout << "homework 4 unit tests" << std::endl;

  std::vector<int> array;
  rand_populate_vector(array);
  std::vector<int> control_array = sorted_vector(array);

  InsertionSortTest(control_array, array);
  SelectionSortTest(control_array, array);
  QuickSortTest(control_array, array);
  MergeSortTest(control_array, array);
  HeapSortTest(control_array, array);
  RadixSortTest(control_array, array);
  BucketSortTest(control_array, array);
  return 0;
}

void rand_populate_vector(std::vector<int> & array) {
  srand(time(NULL));
  for (int i = 0; i < 100; i++) {
    array.push_back(rand() % hundread_thousand + 1);
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

void InsertionSortTest(std::vector<int> control_array, std::vector<int> array) {
  InsertionSort(array);
  assert_eq(control_array, array, "InsertionSortTest");
}

void SelectionSortTest(std::vector<int> control_array, std::vector<int> array) {
  SelectionSort(array);
  assert_eq(control_array, array, "SelectionSortTest");

}
void HeapSortTest(std::vector<int> control_array, std::vector<int> array) {
  HeapSort(array, array.size());
  assert_eq(control_array, array, "HeapSortTest");
}

void MergeSortTest(std::vector<int> control_array, std::vector<int> array) {
  MergeSort(array, 0, array.size() - 1);
  assert_eq(control_array, array, "MergeSortTest");
}

void QuickSortTest(std::vector<int> control_array, std::vector<int> array) {
  QuickSort(array, 0, array.size() - 1);
  assert_eq(control_array, array, "QuickSortTest");
}
void RadixSortTest(std::vector<int> control_array, std::vector<int> array) {}
void BucketSortTest(std::vector<int> control_array, std::vector<int> array) {}
