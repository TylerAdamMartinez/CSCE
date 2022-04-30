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
  array.push_back(976);
  array.push_back(376);
  array.push_back(476);

  cout << "orginal array" << endl;
  print(array);

  RadixSort(array);
  std::cout << "array after RadixSort" << std::endl;
  print(array);
  return 0;
}

void print(std::vector<int> & array) {
  for(int i = 0; i < array.size(); i++ ) {
    std::cout << i << ": " << array.at(i) << std::endl;
  }
}

