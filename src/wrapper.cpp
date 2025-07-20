// osrm_wrapper.cpp
#include <osrm/osrm.hpp>
#include <osrm/table_parameters.hpp>
#include <osrm/engine_config.hpp>
#include <osrm/json_container.hpp>
#include <util/json_renderer.hpp>
#include <osrm/route_parameters.hpp>
#include <osrm/trip_parameters.hpp>

#include <string>
#include <iostream>
#include <cstdlib>

extern "C" {

    struct OSRM_Result {
        int code;
        char* message;
    };

    void* osrm_create(const char* base_path, const char* algorithm) {
        try {
            osrm::EngineConfig config;
            config.storage_config = {base_path};
            config.use_shared_memory = false;

            if (strcmp(algorithm, "CH") == 0) {
                config.algorithm = osrm::EngineConfig::Algorithm::CH;
            }
            else if (strcmp(algorithm, "MLD") == 0) {
                config.algorithm = osrm::EngineConfig::Algorithm::MLD;
            }
            else {
               config.algorithm = osrm::EngineConfig::Algorithm::MLD;
            }

            return new osrm::OSRM(config);
        } catch (const std::exception& e) {
            std::cerr << "Fail to create an OSRM instance: " << e.what() << std::endl;
            return nullptr;
        }
    }

    void osrm_destroy(void* osrm_instance) {
        if (osrm_instance) {
            delete static_cast<osrm::OSRM*>(osrm_instance);
        }
    }

    OSRM_Result osrm_table(void* osrm_instance,
                          const double* coordinates,
                          size_t num_coordinates,
                          const size_t* sources,
                          size_t num_sources,
                          const size_t* destinations,
                          size_t num_destinations) {

        if (!osrm_instance) {
            const char* err = "OSRM instance not found";
            char* msg = new char[strlen(err) + 1];
            strcpy(msg, err);
            return {1, msg};
        }

        osrm::OSRM* osrm_ptr = static_cast<osrm::OSRM*>(osrm_instance);
        osrm::TableParameters params;

        for (size_t i = 0; i < num_coordinates; ++i) {
            params.coordinates.push_back({
                osrm::util::FloatLongitude{coordinates[i * 2]},
                osrm::util::FloatLatitude{coordinates[i * 2 + 1]}
            });
        }

        if (num_sources > 0) {
            params.sources.assign(sources, sources + num_sources);
        }

        if (num_destinations > 0) {
            params.destinations.assign(destinations, destinations + num_destinations);
        }

        osrm::json::Object result;
        const auto status = osrm_ptr->Table(params, result);

        std::string result_str;
        int code;

        if (status == osrm::Status::Ok) {
            code = 0;
            osrm::util::json::render(result_str, result);
        } else {
            code = 1;
            try {
                result_str = std::get<osrm::util::json::String>(result.values.at("message")).value;
            } catch (const std::exception& e) {
                result_str = "Unknown OSRM error";
            }
        }

    char* message = new char[result_str.length() + 1];
    strcpy(message, result_str.c_str());

    return {code, message};
    }

    OSRM_Result osrm_route(void* osrm_instance,
                           const double* coordinates,
                           size_t num_coordinates)
    {
        if (!osrm_instance) {
            const char* err = "OSRM instance not found";
            char* msg = new char[strlen(err) + 1];
            strcpy(msg, err);
            return {1, msg};
        }

        osrm::OSRM* osrm_ptr = static_cast<osrm::OSRM*>(osrm_instance);
        osrm::RouteParameters params;

        for (size_t i = 0; i < num_coordinates; ++i) {
            params.coordinates.push_back({
                osrm::util::FloatLongitude{coordinates[i * 2]},
                osrm::util::FloatLatitude{coordinates[i * 2 + 1]}
            });
        }

        osrm::json::Object result;
        const auto status = osrm_ptr->Route(params, result);

        std::string result_str;
        int code;

        if (status == osrm::Status::Ok) {
            code = 0;
            osrm::util::json::render(result_str, result);
        } else {
            code = 1;
            try {
                result_str = std::get<osrm::util::json::String>(result.values.at("message")).value;
            } catch (const std::exception& e) {
                result_str = "Unknown OSRM error";
            }
        }

        char* message = new char[result_str.length() + 1];
        strcpy(message, result_str.c_str());

        return {code, message};
    }

    OSRM_Result osrm_trip(void* osrm_instance,
                          const double* coordinates,
                          size_t num_coordinates)
    {

            if (!osrm_instance) {
                const char* err = "OSRM instance not found";
                char* msg = new char[strlen(err) + 1];
                strcpy(msg, err);
                return {1, msg};
            }

            osrm::OSRM* osrm_ptr = static_cast<osrm::OSRM*>(osrm_instance);
            osrm::TripParameters params;

            for (size_t i = 0; i < num_coordinates; ++i) {
                params.coordinates.push_back({
                    osrm::util::FloatLongitude{coordinates[i * 2]},
                    osrm::util::FloatLatitude{coordinates[i * 2 + 1]}
                });
            }

            osrm::json::Object result;
            const auto status = osrm_ptr->Trip(params, result);

            std::string result_str;
            int code;

            if (status == osrm::Status::Ok) {
                code = 0;
                osrm::util::json::render(result_str, result);
            } else {
                code = 1;
                try {
                    result_str = std::get<osrm::util::json::String>(result.values.at("message")).value;
                } catch (const std::exception& e) {
                    result_str = "Unknown OSRM error";
                }
            }

            char* message = new char[result_str.length() + 1];
            strcpy(message, result_str.c_str());

            return {code, message};
        }

    void osrm_free_string(char* s) {
        if (s) {
            delete[] s;
        }
    }
}