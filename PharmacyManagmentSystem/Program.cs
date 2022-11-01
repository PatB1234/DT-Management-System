using System;

class Program
{

    static void Main(string[] args)
    {

        login();
        MainMenu();
    }

    public static void login()
    {

        string username = "ILove";
        string password = "Drugs";
        int attempts = 3;

        do
        {

            Console.WriteLine("Enter username:");
            string userInputName = Console.ReadLine();
            Console.WriteLine("Enter password:");
            string userInputPassword = Console.ReadLine();
            if (username == userInputName && password == userInputPassword)
            {

                Console.WriteLine("Accepted... WELCOME!");
                Console.ReadKey();
                break;
            }
            else
            {

                Console.WriteLine("WRONG");
                attempts--;
                Console.WriteLine($"Remaining attempts {attempts}");
                if (attempts == 0)
                {

                    Console.WriteLine("SECURITY ERROR");
                    Environment.Exit(0);
                }

                Console.ReadKey();
                Console.Clear();
            }
        } while (attempts != 0);
    }

    public static void MainMenu()
    {

        Console.Clear();
        Console.WriteLine("Main Menu");
        Console.WriteLine("What would you like to do?");
        Console.WriteLine("1. List All Drugs");
        Console.WriteLine("2. Add a new drug");
        Console.WriteLine("3. Add a existing drug");
        Console.WriteLine("4. Withdraw a Drug");
        Console.WriteLine("5. Exit");
        string choice = Console.ReadLine();

        if (choice == "1") {
            
            ListDrugs();
        } else if (choice == "2") {
            
            AddDrug(false);
        } else if (choice == "3") {
            
            AddDrug(true);
        } else if (choice == "4") {
            
            WithdrawDrug();
        } else if (choice == "5")
        {
            
            Console.WriteLine("Bye Bye!");
            Console.ReadKey();
            Environment.Exit(0);
        }
    }

    public static void ListDrugs()
    {
        
        //Format for drugs is
        //Name|Amount|PackagedDate|ExpiriyDate
        string[] lines = File.ReadAllLines("../../../drugs.txt");
        Console.WriteLine("The current drugs in stock are:");
        for (int i = 0; i < lines.Length; i++)
        {

            string[] splitDrugDetails = lines[i].Split("|");
            Console.WriteLine($"Name: {splitDrugDetails[0]} Amount: {splitDrugDetails[1]} Packaged Date: {splitDrugDetails[2]} Expiry Date: {splitDrugDetails[3]}");
            
        }
        Console.WriteLine("Hit enter to return to main menu: ");
        Console.ReadKey();
        MainMenu();
    }

    public static void AddDrug(bool existing)
    {
        
        Console.WriteLine("Hit enter to return to main menu: ");
        Console.ReadKey();
        MainMenu();
    }

    public static void WithdrawDrug()
    {
        
        Console.WriteLine("Hit enter to return to main menu: ");
        Console.ReadKey();
        MainMenu();
    }
}