#include <mecab.h>

#include <chrono>
#include <cstring>
#include <iostream>
#include <string>
#include <vector>

using namespace MeCab;

int main(int argc, const char **argv) {
    Tagger *tagger = createTagger("-Owakati");

    std::vector<std::string> lines;
    for (std::string line; std::getline(std::cin, line);) {
        lines.push_back(line);
    }
    std::vector<char> buf(65536);
    const std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
    for (std::string &line : lines) {
        std::strcpy(buf.data(), line.c_str());
        const volatime char *r = tagger->parse(buf.data());
    }
    const std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
    const auto d = std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count();
    std::cout
        << "Elapsed-mecab: "
        << static_cast<double>(d) / 1000
        << " [sec]" << std::endl;

    return 0;
}
