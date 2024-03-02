using System;
class Program {
  static void Main(string[] args) {
    string input = Console.ReadLine();

    int res = 0;

    foreach(char ch in input) {
      if(ch=='1') ++res;
    }

    Console.WriteLine(res);
  }
}

