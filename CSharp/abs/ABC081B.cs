using System;

public class Program {
  public static void Main(string[] args) {
    Console.ReadLine();
    string[] numstrs = Console.ReadLine().Split(' ');
    int[] nums = new int[numstrs.Length];

    for(int i = 0; i < nums.Length; ++i) {
      nums[i] = int.Parse(numstrs[i]);
    }

    int count = ShiftOnly(nums);

    Console.WriteLine(count);
  }

  public static int ShiftOnly(int[] nums) {
    int count = 0;

    while(true) {
      for(int i = 0; i < nums.Length; ++i) {
        if(nums[i] % 2 != 0) {
          return count;
        }

        nums[i] /= 2;

        if(i == nums.Length-1) {
          ++count;
        }
      }
    }
  }
}


