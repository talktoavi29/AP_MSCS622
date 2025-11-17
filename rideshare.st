"Ride Sharing System in Smalltalk (GNU Smalltalk style)
 Paste this into JDoodle with language = Smalltalk and click Run."

"========================"
" Class Definitions      "
"========================"

Object subclass: #Ride
    instanceVariableNames: 'rideID pickupLocation dropoffLocation distance'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'!

!Ride class methodsFor: 'instance creation'!

newWithID: anID pickup: pickup dropoff: dropoff distance: dist
    ^ self new
        initializeWithID: anID
        pickup: pickup
        dropoff: dropoff
        distance: dist!

!!

!Ride methodsFor: 'initialization'!

initializeWithID: anID pickup: pickup dropoff: dropoff distance: dist
    rideID := anID.
    pickupLocation := pickup.
    dropoffLocation := dropoff.
    distance := dist.
    ^ self!

!!

!Ride methodsFor: 'accessing'!

distance
    ^ distance!

!!

!Ride methodsFor: 'computing'!

fare
    "Abstract method – subclasses must override."
    self subclassResponsibility!

rideDetails
    Transcript
        show: 'Ride ID: '; show: rideID printString; cr;
        show: 'Pickup: '; show: pickupLocation; cr;
        show: 'Dropoff: '; show: dropoffLocation; cr;
        show: 'Distance: '; show: distance printString; show: ' miles'; cr;
        show: 'Fare: $'; show: self fare printString; cr.
    Transcript cr!

!!

"------------------------"
" StandardRide subclass  "
"------------------------"

Ride subclass: #StandardRide
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'!

!StandardRide methodsFor: 'computing'!

fare
    | baseFare perMile |
    baseFare := 2.5.
    perMile := 1.25.
    ^ baseFare + (perMile * self distance)!

rideDetails
    Transcript show: '--- Standard Ride ---'; cr.
    super rideDetails!

!!

"------------------------"
" PremiumRide subclass   "
"------------------------"

Ride subclass: #PremiumRide
    instanceVariableNames: ''
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'!

!PremiumRide methodsFor: 'computing'!

fare
    | baseFare perMile |
    baseFare := 5.0.
    perMile := 2.25.
    ^ baseFare + (perMile * self distance)!

rideDetails
    Transcript show: '*** Premium Ride ***'; cr.
    super rideDetails!

!!

"------------------------"
" Driver class           "
"------------------------"

Object subclass: #Driver
    instanceVariableNames: 'driverID name rating assignedRides'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'!

!Driver class methodsFor: 'instance creation'!

newWithID: anID name: aName rating: aRating
    ^ self new
        initializeWithID: anID
        name: aName
        rating: aRating!

!!

!Driver methodsFor: 'initialization'!

initializeWithID: anID name: aName rating: aRating
    driverID := anID.
    name := aName.
    rating := aRating.
    assignedRides := OrderedCollection new.
    ^ self!

!!

!Driver methodsFor: 'behavior'!

addRide: aRide
    assignedRides add: aRide!

getDriverInfo
    Transcript
        show: 'Driver ID: '; show: driverID printString; cr;
        show: 'Name: '; show: name; cr;
        show: 'Rating: '; show: rating printString; cr;
        show: 'Total Rides: '; show: assignedRides size printString; cr; cr.

    assignedRides do: [:ride |
        ride rideDetails ].
    Transcript cr!

!!

"------------------------"
" Rider class            "
"------------------------"

Object subclass: #Rider
    instanceVariableNames: 'riderID name requestedRides'
    classVariableNames: ''
    poolDictionaries: ''
    category: 'RideSharing'!

!Rider class methodsFor: 'instance creation'!

newWithID: anID name: aName
    ^ self new
        initializeWithID: anID
        name: aName!

!!

!Rider methodsFor: 'initialization'!

initializeWithID: anID name: aName
    riderID := anID.
    name := aName.
    requestedRides := OrderedCollection new.
    ^ self!

!!

!Rider methodsFor: 'behavior'!

requestRide: aRide
    requestedRides add: aRide!

viewRides
    Transcript
        show: 'Rider ID: '; show: riderID printString; cr;
        show: 'Name: '; show: name; cr;
        show: 'Requested Rides: '; show: requestedRides size printString; cr; cr.

    requestedRides do: [:ride |
        ride rideDetails ].
    Transcript cr!

!!

"========================"
" Sample Usage / Demo    "
"========================"

| r1 r2 r3 driver rider rides |

r1 := StandardRide
        newWithID: 101
        pickup: 'Downtown'
        dropoff: 'Airport'
        distance: 15.0.

r2 := PremiumRide
        newWithID: 102
        pickup: 'University'
        dropoff: 'Mall'
        distance: 8.0.

r3 := StandardRide
        newWithID: 103
        pickup: 'Office'
        dropoff: 'Home'
        distance: 5.5.

rides := OrderedCollection with: r1 with: r2 with: r3.

Transcript show: 'All Rides (Polymorphic Call):'; cr; cr.
rides do: [:each |
    each rideDetails ].

driver := Driver newWithID: 1 name: 'Alice Anderson' rating: 4.9.
driver addRide: r1.
driver addRide: r2.

rider := Rider newWithID: 1001 name: 'Bob Brown'.
rider requestRide: r1.
rider requestRide: r3.

Transcript cr; show: '==== Driver Info ===='; cr.
driver getDriverInfo.

Transcript cr; show: '==== Rider Info ===='; cr.
rider viewRides.

Transcript show: 'Done.'; cr.
