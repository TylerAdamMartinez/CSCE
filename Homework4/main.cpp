#include <iostream>
#include <vector>

// custom header file for sorting funcs
#include "lib.h"

int main() {
  using namespace std;
  cout << "homework 4 main program" << endl;

  vector<int> array;
  array.push_back(4);
  array.push_back(432);
  array.push_back(53);
  array.push_back(238);
  array.push_back(2);
  array.push_back(65);
  array.push_back(165);
  array.push_back(876);

  cout << "orginal array" << endl;
  for( auto i: array) {
    cout << i << endl;
  }

  HeapSort(array, array.size() - 1);
  std::cout << "array after MergeSort" << std::endl;
  for( auto i: array) {
    cout << i << endl;
  }

  return 0;
}
