#include <fstream> // for file-access
#include <string>
#include <array>
#include <iostream>
#include <sstream>
#define NUM_DAY 256

using namespace std;

int main(int argc, char *argv[])
{
    std::array<unsigned long long int, 10> population = {0};
    if (argc <= 1)
    {
        cout << "No file name entered. Exiting...";
        return -1;
    }
    ifstream infile(argv[1]); //open the file
    if (infile.is_open() && infile.good())
    {
        string line = "";
        while (getline(infile, line, ','))
        {
            std::stringstream ss(line);
            int readAge = 0;
            ss >> readAge;
            population[readAge]++;
        }
    }
    else
        cout << "Failed to open file..";
    for (int i_day = 0; i_day < NUM_DAY; ++i_day)
    {
        population[9] = population[0];
        for (int i_age = 0; i_age < 9; i_age++)
            population[i_age] = population[i_age + 1];
        population[6] += population[9];
    }
    unsigned long long int pop_size = 0;
    for (int i_age = 0; i_age < 9; i_age++)
        pop_size += population[i_age];
    cout << "Population size : " << pop_size << endl;
    return 0;
}