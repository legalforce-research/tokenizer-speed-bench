#include <mecab.h>

#include <iostream>
#include <string>
#include <vector>

using namespace MeCab;

int main(int argc, const char **argv) {
    Tagger *tagger = createTagger("-Owakati");

    int n_words = 0;
    std::vector<std::string> lines;
    for (std::string line; std::getline(std::cin, line);) {
        lines.push_back(line);
    }
    std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
    std::vector<char> buf(8192);
    for (std::string &line : lines) {
        std::strncpy(buf.data(), line.c_str(), line.size());
        const char *r = tagger->parse(buf.data());
        n_words += static_cast<int>(reinterpret_cast<size_t>(r));
    }
    std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
    auto d = std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count();
    std::cout
        << "Elapsed-mecab: "
        << static_cast<double>(d) / 1000
        << " [sec]" << std::endl;
    std::cout << n_words << std::endl;

    return 0;
}
