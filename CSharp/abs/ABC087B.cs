using System;

public class Program {
  public static void Main(string[] args) {
    int num500 = int.Parse(Console.ReadLine());
    int num100 = int.Parse(Console.ReadLine());
    int num50 = int.Parse(Console.ReadLine());
    int price = int.Parse(Console.ReadLine());

    int count = CountCoinPatterns(num500, num100, num50, price);

    Console.WriteLine(count);
  }

  private static int CountCoinPatterns(int num500, int num100, int num50, int price) {
    int count = 0;

    int num500max = price / 500;
    if(num500max > num500) num500max = num500;

    int num100max = price / 100;
    if(num100max > num100) num100max = num100;

    int num50max = price / 50;
    if(num50max > num50) num50max = num50;

    for(int num500test = 0; num500test <= num500max; ++num500test) {
      for(int num100test = 0; num100test <= num100max; ++num100test) {
        for(int num50test = 0; num50test <= num50max; ++num50test) {
          if(500*num500test + 100*num100test + 50*num50test == price) {
            ++count;
          }
        }
      }
    }

    return count;

  }
}


