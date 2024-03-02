#include<bits/stdc++.h>
using namespace std;
constexpr char black = '#';
constexpr char white = '.';

void next(vector<vector<char>> & data, int height, int width);
bool isAllBlack(vector<vector<char>> const& data, int height, int width);

int main(){
  int height, width;
  cin >> height >> width;
  
  vector<vector<char>> data = vector<vector<char>>(height, vector<char>(width));
  for(int row=0; row<height; ++row){
    for(int col=0; col<width; ++col){
      char in;
      cin >> in;
      data[row][col] = in;
    }
  }
  
  int count = 0;
  while(!isAllBlack(data, height, width)){
    next(data, height, width);
    ++count;
  }
  
  cout << count;
}

void next(vector<vector<char>> & data, int height, int width){
  vector<vector<char>> bak = vector<vector<char>>(height, vector<char>(width));
  for(int row=0; row<height; ++row){
    for(int col=0; col<width; ++col){
      bak[row][col] = data[row][col];
    }
  }
  
  for(int row=0; row<height; ++row){
    for(int col=0; col<width; ++col){
      if(bak[row][col]==black){
        if(row != height-1) data[row+1][col] = black;
        if(row != 0)        data[row-1][col] = black;
        if(col != width-1)  data[row][col+1] = black;
        if(col != 0)        data[row][col-1] = black;
      }
    }
  }
}

bool isAllBlack(vector<vector<char>> const& data, int height, int width){
  for(int row=0; row<height; ++row){
    for(int col=0; col<width; ++col){
      if(data[row][col]==white){
        return false;
      }
    }
  }
  return true;
}

