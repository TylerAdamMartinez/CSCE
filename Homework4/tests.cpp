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

  myFirstTest();
  return 0;
}

void myFirstTest() {
  using namespace std;

  vector<int> myvec;
  myvec.push_back(45);
  vector<int> mysecondvec;
  mysecondvec.push_back(45);
  assertm(myvec == mysecondvec, "failed here..");
  cout << "test was successful" << endl;
}

void InsertionSortTest() {}
void SelectionSortTest() {}
void HeapSortTest() {}
void MergeSortTest() {}
void QuickSortTest() {}
void RadixSortTest() {}
void BucketSortTest() {}
