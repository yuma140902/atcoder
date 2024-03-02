using System;
class Program {
  static void Main(string[] args) {
    string[] nums = Console.ReadLine().Split(' ');

    int a = int.Parse(nums[0]);
    int b = int.Parse(nums[1]);

    Console.WriteLine((a*b)%2==0 ? "Even" : "Odd");
  }
}
