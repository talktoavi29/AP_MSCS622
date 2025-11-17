#include <iostream>
#include <string>
#include <vector>
#include <memory>
#include <iomanip>

// ------------------------
// Base class: Ride
// ------------------------
class Ride {
protected:
    int rideID;
    std::string pickupLocation;
    std::string dropoffLocation;
    double distance; // in miles

public:
    Ride(int id,
         const std::string& pickup,
         const std::string& dropoff,
         double dist)
        : rideID(id),
          pickupLocation(pickup),
          dropoffLocation(dropoff),
          distance(dist) {}

    virtual ~Ride() = default;

    // Polymorphic fare calculation
    virtual double fare() const = 0;

    // Virtual so subclasses can customize the header/label
    virtual void rideDetails() const {
        std::cout << "Ride ID: " << rideID
                  << "\nPickup: " << pickupLocation
                  << "\nDropoff: " << dropoffLocation
                  << "\nDistance: " << distance << " miles"
                  << "\nFare: $" << std::fixed << std::setprecision(2) << fare()
                  << "\n";
    }
};

// ------------------------
// StandardRide subclass
// ------------------------
class StandardRide : public Ride {
public:
    StandardRide(int id,
                 const std::string& pickup,
                 const std::string& dropoff,
                 double dist)
        : Ride(id, pickup, dropoff, dist) {}

    double fare() const override {
        const double baseFare = 2.50;
        const double perMile = 1.25;
        return baseFare + perMile * distance;
    }

    void rideDetails() const override {
        std::cout << "[Standard Ride]\n";
        Ride::rideDetails();
        std::cout << "----------------------\n";
    }
};

// ------------------------
// PremiumRide subclass
// ------------------------
class PremiumRide : public Ride {
public:
    PremiumRide(int id,
                const std::string& pickup,
                const std::string& dropoff,
                double dist)
        : Ride(id, pickup, dropoff, dist) {}

    double fare() const override {
        const double baseFare = 5.00;
        const double perMile = 2.25;
        return baseFare + perMile * distance;
    }

    void rideDetails() const override {
        std::cout << "[Premium Ride]\n";
        Ride::rideDetails();
        std::cout << "----------------------\n";
    }
};

// ------------------------
// Driver class
// ------------------------
class Driver {
private:
    int driverID;
    std::string name;
    double rating;
    std::vector<std::shared_ptr<Ride>> assignedRides; // encapsulated

public:
    Driver(int id, const std::string& n, double r)
        : driverID(id), name(n), rating(r) {}

    void addRide(const std::shared_ptr<Ride>& ride) {
        assignedRides.push_back(ride);
    }

    void getDriverInfo() const {
        std::cout << "Driver ID: " << driverID
                  << "\nName: " << name
                  << "\nRating: " << rating
                  << "\nTotal Rides: " << assignedRides.size() << "\n\n";

        std::cout << "Driver Ride History:\n";
        for (const auto& ride : assignedRides) {
            ride->rideDetails(); // polymorphic call
        }
        std::cout << "========================\n";
    }
};

// ------------------------
// Rider class
// ------------------------
class Rider {
private:
    int riderID;
    std::string name;
    std::vector<std::shared_ptr<Ride>> requestedRides; // encapsulated

public:
    Rider(int id, const std::string& n)
        : riderID(id), name(n) {}

    void requestRide(const std::shared_ptr<Ride>& ride) {
        requestedRides.push_back(ride);
    }

    void viewRides() const {
        std::cout << "Rider ID: " << riderID
                  << "\nName: " << name
                  << "\nRequested Rides: " << requestedRides.size() << "\n\n";

        std::cout << "Rider Ride History:\n";
        for (const auto& ride : requestedRides) {
            ride->rideDetails(); // polymorphic call
        }
        std::cout << "========================\n";
    }
};

// ------------------------
// Main: demo / sample output
// ------------------------
int main() {
    // Create some rides
    std::shared_ptr<Ride> r1 = std::make_shared<StandardRide>(
        101, "Downtown", "Airport", 15.0
    );
    std::shared_ptr<Ride> r2 = std::make_shared<PremiumRide>(
        102, "University", "Mall", 8.0
    );
    std::shared_ptr<Ride> r3 = std::make_shared<StandardRide>(
        103, "Office", "Home", 5.5
    );

    // Demonstrate polymorphism with a collection of base-class pointers
    std::vector<std::shared_ptr<Ride>> rides = { r1, r2, r3 };

    std::cout << "All Rides (Polymorphic Calls):\n";
    std::cout << "========================\n";
    for (const auto& ride : rides) {
        ride->rideDetails(); // virtual dispatch to Standard/Premium
    }

    // Create a driver and a rider
    Driver d1(1, "Alice Anderson", 4.9);
    d1.addRide(r1);
    d1.addRide(r2);

    Rider rider1(1001, "Bob Brown");
    rider1.requestRide(r1);
    rider1.requestRide(r3);

    std::cout << "\nDriver Info:\n";
    std::cout << "========================\n";
    d1.getDriverInfo();

    std::cout << "Rider Info:\n";
    std::cout << "========================\n";
    rider1.viewRides();

    return 0;
}
