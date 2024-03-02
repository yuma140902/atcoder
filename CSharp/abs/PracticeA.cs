using System;
class Program {
  static void Main(string[] args) {
    int a = int.Parse(Console.ReadLine());

    string[] nums = Console.ReadLine().Split(' ');
    int b = int.Parse(nums[0]);
    int c = int.Parse(nums[1]);

    string str = Console.ReadLine();

    Console.WriteLine("{0} {1}", (a+b+c), str);
  }
}

