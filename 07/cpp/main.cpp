#include <fstream> // for file-access
#include <string>
#include <limits>
#include <vector>
#include <iostream>
#include <sstream>


using namespace std;

int main(int argc, char *argv[])
{
    std::vector<int> position_list;
    std::vector<int> consomption_cost;
    int max_position = 0;
    if (argc <= 1)
    {
        cout << "No file name entered. Exiting...";
        return -1;
    }
    ifstream infile(argv[1]); // open the file
    if (infile.is_open() && infile.good())
    {
        string line = "";
        while (getline(infile, line, ','))
        {
            std::stringstream ss(line);
            int read_position = 0;
            ss >> read_position;
            position_list.push_back(read_position);
            if(read_position>max_position) max_position=read_position;
        }
    }
    else
        cout << "Failed to open file..";
    consomption_cost.resize(max_position+1);
    consomption_cost[0]=0;
    for (int i_dis = 1; i_dis <= max_position; i_dis++)
    {
        //pb_a
        //consomption_cost[i_dis] = i_dis;
        //pb_b
        consomption_cost[i_dis] = consomption_cost[i_dis-1]+i_dis;
    }
    int min_pos  =  std::numeric_limits<int>::max();
    int min_fuel =  std::numeric_limits<int>::max();
    for (int i_test = 0; i_test < position_list.size(); i_test++)
    {
        int fuel = 0;
        for (int i_pos = 0; i_pos < position_list.size(); i_pos++)
        {
            int dis = abs(position_list[i_pos] - i_test);
            fuel += consomption_cost[dis];
        }
        if(fuel<min_fuel)
        {
            min_pos = i_test;
            min_fuel = fuel;
        }
    }
    cout << "optimal position : " << min_pos << endl;
    cout << "consumption      : " << min_fuel << endl;
    return 0;
}