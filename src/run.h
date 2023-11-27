#pragma once
#include <vector>

using DVec = std::vector<double>;

struct ECPData {
  std::vector<double> integrals;
  // output nested vec in ffi is hard, so we use 1-d vec here for workaround
  std::vector<double> first_derivs;
};


ECPData test_ecpint(std::string share_dir);


class ECPIntWrapper {
public:
    ECPIntWrapper(std::string share_dir) {
      data = test_ecpint(share_dir);
    }

    std::vector<double> get_integrals() {
        return data.integrals;
    }

    std::vector<double> get_first_derivs () {
        return data.first_derivs;
    }

private:
    ECPData data;
};

