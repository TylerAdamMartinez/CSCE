#include <iostream>
#include <vector>
// custom header files for sorting & unit tests funcs
#include "lib.h"
#include "tests.h"
// testing purposes
#include <cassert>
#define assertm(exp, msg) assert(((void)msg, exp))


int main() {
  using namespace std;
  cout << "homework 4 unit tests" << endl;

  InsertionSortTest();
  SelectionSortTest();
  return 0;
}

void InsertionSortTest() {
  using namespace std;

  vector<int> control_array;
  control_array.push_back(2);
  control_array.push_back(4);
  control_array.push_back(53);
  control_array.push_back(65);
  control_array.push_back(165);
  control_array.push_back(238);
  control_array.push_back(432);
  control_array.push_back(876);

  vector<int> array;
  array.push_back(4);
  array.push_back(432);
  array.push_back(53);
  array.push_back(238);
  array.push_back(2);
  array.push_back(65);
  array.push_back(165);
  array.push_back(876);

  InsertionSort(array);
  assertm(control_array == array, "InsertionSortTest [failed]");
  cout << "InsertionSortTest [passed]" << endl;

}

void SelectionSortTest() {
  using namespace std;

  vector<int> control_array;
  control_array.push_back(2);
  control_array.push_back(4);
  control_array.push_back(53);
  control_array.push_back(65);
  control_array.push_back(165);
  control_array.push_back(238);
  control_array.push_back(432);
  control_array.push_back(876);

  vector<int> array;
  array.push_back(4);
  array.push_back(432);
  array.push_back(53);
  array.push_back(238);
  array.push_back(2);
  array.push_back(65);
  array.push_back(165);
  array.push_back(876);

  SelectionSort(array);
  assertm(control_array == array, "SelectionSortTest [failed]");
  cout << "SelectionSortTest [passed]" << endl;

}
void HeapSortTest() {}
void MergeSortTest() {}
void QuickSortTest() {}
void RadixSortTest() {}
void BucketSortTest() {}
