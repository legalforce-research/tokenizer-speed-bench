#include <kytea/kytea.h>
#include <kytea/kytea-struct.h>
#include <kytea/string-util.h>

#include <chrono>
#include <iostream>
#include <string>

using namespace kytea;

int main(int argc, const char **argv) {
    KyteaConfig *config = new KyteaConfig;
    config->setDebug(0);
    config->setOnTraining(false);
    config->parseRunCommandLine(argc, argv);

    Kytea *kytea = new Kytea(config);

    kytea->readModel(config->getModelFile().c_str());
    StringUtil *util = kytea->getStringUtil();

    int n_words = 0;
    std::vector<std::string> lines;
    for (std::string line; std::getline(std::cin, line);) {
        lines.push_back(line);
    }
    std::chrono::steady_clock::time_point begin = std::chrono::steady_clock::now();
    for (std::string line : lines) {
        KyteaString surface_string = util->mapString(line);
        KyteaSentence sentence(surface_string, surface_string);
        kytea->calculateWS(sentence);
        n_words += sentence.words.size();
    }
    std::chrono::steady_clock::time_point end = std::chrono::steady_clock::now();
    auto d = std::chrono::duration_cast<std::chrono::milliseconds>(end - begin).count();
    std::cout
        << "Elapsed-kytea: "
        << static_cast<double>(d) / 1000
        << " [sec]" << std::endl;
    std::cerr << n_words << std::endl;

    return 0;
}
