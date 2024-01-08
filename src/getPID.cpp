#include<Windows.h>
#include<iostream>
#include <fstream>
#include"json.hpp"

int main(){
    nlohmann::json obj;
    HWND hwnd = FindWindowA(("LWJGL"), nullptr);
    DWORD pID;
    GetWindowThreadProcessId(hwnd, &pID);
    
    if(hwnd != nullptr){
        if(pID == 0){
            std::cout << "unable to find process" << std::endl;
        } else{
            std::cout << "process found, pid: " << pID << std::endl;

            obj["pid"] = pID;
            std::string json_str = obj.dump();
            const std::string file_name = "info.json";
            std::ofstream file_out(file_name);

            if (file_out.is_open()) {
                // Escribir la cadena JSON en el archivo
                file_out << json_str;
                std::cout << "json writen in " << file_name << std::endl;
                // Cerrar el archivo
                file_out.close();
            } else {
                std::cerr << "error trying to open " << file_name << std::endl;
            }
        }
    }

    else{
        std::cout << "unable to find window" << std::endl;
    }

}