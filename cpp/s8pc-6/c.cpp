#include <bits/stdc++.h>
using namespace std;

constexpr char black = '#';
constexpr char white = '.';

bool canGoThrough(vector<vector<char>> const& data, int row);
bool canGoDown(vector<vector<char>> const& data);

int main(){
  int height, width;
  cin >> height >> width;
  
  vector<vector<char>> data(height, vector<char>(width));
  for(int row=0; row<height; ++row){
    for(int col=0; col<width; ++col){
      char in;
      cin >> in;
      data[row][col] = in;
    }
  }
  
  
  bool gothrough = false;
  for(int row=0; row<height; ++row){
    if(canGoThrough(data, row)){
      gothrough = true;
    }
  }
  if(gothrough && canGoDown(data)){
    cout << "Yay!" << endl;
  }
  else{
    cout << ":(" << endl;
  }
  
  return 0;
}

bool canGoThrough(vector<vector<char>> const& data, int row) {
  int colsize = data[0].size();
  for(int col=0; col<colsize; ++col) {
    if(data[row][col] == black) return false;
  }
  return true;
}

bool canGoDown(vector<vector<char>> const& data) {
  int rowsize = data.size() + 1;
  int colsize = data[0].size() + 1;
  
  vector<vector<bool>> table
    (rowsize, vector<bool>(colsize, false));
  table[1][1] = true;
  
  for(int row=1; row<rowsize; ++row){
    for(int col=1; col<colsize; ++col){
      if(row==1 && col==1) continue;
      else{
        table[row][col] = 
          data[row-1][col-1] == white &&
          (table[row-1][col] || table[row][col-1]);
      }
      //cout << boolalpha << "(" << row << ", " << col << ")" << table[row][col] << endl;
    }
  }
  
  return table[rowsize-1][colsize-1];
  
}

