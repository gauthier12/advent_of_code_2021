#include <fstream> // for file-access
#include <string>
#include <vector>
#include <array>
#include <iostream>
#include <iomanip>
#include <sstream>
#include <cstring>

using namespace std;
#define GRID_SIZE 5

class Board
{
public:
    int value[GRID_SIZE][GRID_SIZE];
    int checked[GRID_SIZE][GRID_SIZE];
    Board()
    {
        Reset();
    }
    void Reset()
    {
        for (int i = 0; i < GRID_SIZE; ++i)
            for (int j = 0; j < GRID_SIZE; ++j)
            {
                value[j][i] = 0;
                checked[j][i] = false;
            }
    }
    void Mark(int val)
    {
        for (int i = 0; i < GRID_SIZE; ++i)
            for (int j = 0; j < GRID_SIZE; ++j)
            {
                if (val == value[j][i])
                {
                    checked[j][i] = true;
                }
            }
    }
    bool CheckLine(int i_line)
    {
        bool winner = true;
        for (int j = 0; j < GRID_SIZE; ++j)
        {
            winner = winner && checked[i_line][j];
        }
        return winner;
    }
    bool CheckRow(int j_row)
    {
        bool winner = true;
        for (int i = 0; i < GRID_SIZE; ++i)
        {
            winner = winner && checked[i][j_row];
        }
        return winner;
    }
    bool CheckBoard()
    {
        bool winner = false;
        for (int i = 0; i < GRID_SIZE; ++i)
        {
            winner = winner || CheckRow(i) || CheckLine(i);
            ;
        }
        return winner;
    }
    int ComputeScore()
    {
        if (!CheckBoard())
        {
            return 0;
        }
        int sum = 0;
        for (int i = 0; i < GRID_SIZE; ++i)
            for (int j = 0; j < GRID_SIZE; ++j)
            {
                if (!checked[j][i])
                {
                    sum += value[j][i];
                }
            }
        return sum;
    }
    void Print()
    {
        for (int i = 0; i < GRID_SIZE; ++i)
        {
            for (int j = 0; j < GRID_SIZE; ++j)
            {
                cout << std::setfill('0') << std::setw(2) << value[j][i] << " ";
            }
            cout << "\t";

            for (int j = 0; j < GRID_SIZE; ++j)
            {
                cout << "|";
                if (checked[j][i])
                    cout << "x";
                else
                    cout << " ";
            }
            cout << "|" << endl;
        }
    }
};

int main(int argc, char *argv[])
{
    string first_line;
    string filename = "../input";
    std::vector<Board> boardList;
    std::vector<int> numberList;
    if (argc <= 1)
    {
        cout << "No file name entered. ";
    }
    else
    {
        filename = string(argv[1]);
    }
    ifstream infile(filename); //open the file
    if (infile.is_open() && infile.good())
    {
        getline(infile, first_line);
        {
            stringstream ss(first_line);
            for (int i; ss >> i;)
            {
                numberList.push_back(i);
                if (ss.peek() == ',')
                    ss.ignore();
            }
        }
        Board tempBoard;
        string line = "";
        int curLine = 0;
        int curRow = 0;
        while (infile.good())
        {
            infile >> tempBoard.value[curLine][curRow++];
            if (curRow == GRID_SIZE)
            {
                curLine++;
                curRow = 0;
            }
            if (curLine == GRID_SIZE)
            {
                curLine = 0;
                curRow = 0;
                boardList.push_back(tempBoard);
            }
        }
    }
    else
        cout << "Failed to open file..";

    cout << "Number of board " << boardList.size() << endl;
    cout << "Beginning the game " << numberList.size() << endl;
    bool won = false;
    for (int i_val = 0; i_val < numberList.size(); i_val++)
    {
        int j_val = numberList[i_val];
        cout << "new number " << std::setfill('0') << std::setw(2) << i_val << "/" << numberList.size() << " : " << j_val << endl;
        for (int i_board = 0; i_board != boardList.size(); i_board++)
        {
            boardList[i_board].Mark(j_val);
            if (boardList[i_board].CheckBoard())
            {
                int score = j_val * boardList[i_board].ComputeScore();
                if (score != 0)
                {
                    cout << "We have a winner " << i_board << endl;
                    boardList[i_board].Print();
                    cout << "score : " << score << endl;
                    boardList[i_board].Reset();
                }
            }
        }
    }
    return 0;
}