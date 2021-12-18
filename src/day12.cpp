//
// Created by jovasa on 2021-12-17.
//
#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>
#include <map>

void add_new_cave(std::vector<std::string> &lowercase, std::vector<std::string> &uppercase, const std::string &first);

uint16_t get_index(const std::vector<std::string> &lowercase, const std::vector<std::string> &uppercase, const std::string &first);

constexpr uint16_t visited_small_cave = 1 << 15;

int main() {
    std::ifstream file("data/day12.txt");
    if (file.is_open()) {
        std::string line;
        std::vector<std::string> lowercase{};
        std::vector<std::string> uppercase{};
        lowercase.emplace_back("start");
        lowercase.emplace_back("end");
        std::map<uint16_t , std::vector<uint16_t> > edges{};
        while (std::getline(file, line)) {
            // using printf() in all tests for consistency
            unsigned long pos = line.find('-');
            const std::string first = line.substr(0, pos);
            add_new_cave(lowercase, uppercase, first);
            const std::string second = line.substr(pos + 1,line.length());
            add_new_cave(lowercase, uppercase, second);

        }
        file.clear();
        file.seekg(0, std::ifstream::beg);
        while (std::getline(file, line)) {
            unsigned long pos = line.find('-');
            const std::string first = line.substr(0, pos);
            const std::string second = line.substr(pos + 1,line.length());
            uint16_t first_as_int = get_index(lowercase, uppercase, first);
            uint16_t second_as_int = get_index(lowercase, uppercase, second);
            if(first != "end" && second != "start") {
                edges[first_as_int].emplace_back(second_as_int);
            }
            if (first != "start" && second != "end") {
                edges[second_as_int].emplace_back(first_as_int);
            }
        }
        file.close();

        uint32_t large_cave_mask = ~0u << (32 - uppercase.size());
        large_cave_mask >>= (32 - uppercase.size());
        large_cave_mask <<= (lowercase.size());
        const int number_of_caves = lowercase.size() + uppercase.size();
        
        uint64_t total = 0;
        
        std::map<uint16_t, std::vector<uint16_t>> work_set{};
        work_set[1].emplace_back(1);
        while (!work_set.empty()) {
            std::map<uint16_t, std::vector<uint16_t>> temp{};
            for (int i = 0; i < number_of_caves; ++i) {
                for(uint16_t next_cave : edges[1 << i]) {
                    for (auto path: work_set[1 << i]) {
                        if(next_cave == 2) {
                            total += 1; 
                            continue;
                        }
                        bool large_cavern = next_cave & large_cave_mask;
                        bool not_visited_before = (path & next_cave) == 0;
                        if (large_cavern || not_visited_before || !(path & visited_small_cave)) {
                            const uint16_t has_visited_small = !large_cavern && !not_visited_before;
                            if (has_visited_small) path |= visited_small_cave;
                            path |= next_cave;
                            temp[next_cave].emplace_back(path);
                        }

                    }
                }
            }
            work_set = std::move(temp);
        }
        std::cout << total << '\n';
    }
}

uint16_t get_index(const std::vector<std::string> &lowercase, const std::vector<std::string> &uppercase, const std::string &first) {
     auto p = std::find(lowercase.begin(), lowercase.end(), first);
    if (p != lowercase.end()) {
        return  1 << (p - lowercase.begin());
    } else {
        return (1 << (std::find(uppercase.begin(), uppercase.end(), first) - uppercase.begin() + lowercase.size()) );
    }
}

void add_new_cave(std::vector<std::string> &lowercase, std::vector<std::string> &uppercase, const std::string &first) {
    if (first == "start" or first == "end") return;
    if (islower(first[0]) && std::find(lowercase.begin(), lowercase.end(), first) == lowercase.end())
        lowercase.emplace_back(first);
    else if (!islower(first[0]) && std::find(uppercase.begin(), uppercase.end(), first) == uppercase.end())
        uppercase.emplace_back(first);
}
