#include <iostream>
#include <vector>
// custom header file for sorting funcs
#include "lib.h"
// testing purposes
#include <cassert>
#define assertm(exp, msg) assert(((void)msg, exp))
#


int main() {
  using namespace std;
  cout << "homework 4 unit tests" << endl;

  vector<int> myvec;
  myvec.push_back(45);
  vector<int> mysecondvec;
  mysecondvec.push_back(45);
  assertm(myvec == mysecondvec, "failed here..");
  cout << "test was successful" << endl;
  return 0;
}
