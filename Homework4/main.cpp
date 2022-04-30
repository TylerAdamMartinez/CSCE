#include <iostream>
#include <vector>

// custom header file for sorting funcs
#include "lib.h"

void print(std::vector<int> & array);

int main() {
  using namespace std;
  cout << "homework 4 main program" << endl;

  vector<int> array;
  array.push_back(4);
  array.push_back(53);
  array.push_back(238);
  array.push_back(2);
  array.push_back(65);
  array.push_back(165);
  array.push_back(876);

  cout << "orginal array" << endl;
  print(array);

  QuickSort(array, 0, array.size() - 1);
  std::cout << "array after QuickSort" << std::endl;
  print(array);
  return 0;
}

void print(std::vector<int> & array) {
  for(int i = 0; i < array.size(); i++ ) {
    std::cout << array.at(i) << std::endl;
  }
}

