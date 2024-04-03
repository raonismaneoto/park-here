import React, { useEffect, useState } from "react";
import {
  StyleSheet,
  View,
  TouchableOpacity,
  Text,
  Dimensions,
  Image,
} from "react-native";
import { useNavigation } from "@react-navigation/native";
import * as Location from "expo-location";
import MapView, { Marker } from "react-native-maps";
import axios from "axios";

interface Region {
  latitude: number,
  longitude: number,
  latitudeDelta: number,
  longitudeDelta: number
}

interface Vacancy {
  id: string,
  region: Region
}

const Navigation = () => {
  const [currentLocation, setCurrentLocation] = useState<Location.LocationObjectCoords>({latitude: 0, longitude: 0, altitude: null, accuracy: null, altitudeAccuracy: null,heading: null, speed: null});
  const [initialRegion, setInitialRegion] = useState<Region>({latitude: 0, longitude: 0, latitudeDelta: 0, longitudeDelta:0});
  const [vacancies, setVacancies] = useState<Vacancy[]>([]);

  const isCurrLocationSet = () => {
    return currentLocation.latitude !== 0 && currentLocation.longitude !== 0;
  }

  const isInitialRegionSet = () => initialRegion.latitude !== 0 && initialRegion.longitude !== 0;

  useEffect(() => {
    const getLocation = async () => {
      let { status } = await Location.requestForegroundPermissionsAsync();
      console.log(status);

      if (status !== "granted") {
        console.log("Permission to access location was denied");
        return;
      }

      await Location.watchPositionAsync({
        accuracy:Location.Accuracy.High,
        timeInterval: 10000,
        distanceInterval: 50,
      }, (newLocation) => {
        setCurrentLocation(newLocation.coords);
        setInitialRegion({
          ...newLocation.coords,
          latitudeDelta: 0.1,
          longitudeDelta: 0.1,
        });
      });
    };

    getLocation();
  }, []);

  useEffect(() => {
    const getVacancies = async () => {
      console.log("going to call the backend")
      try {
        const response = await axios.get(`http://10.0.2.2:8000/api/park-here/vacancies/search?latitude=-11.301732&longitude=-41.85516&radius=0&vacancy_type=motorcycle`);
        console.log(response.data);
        setVacancies(response.data);
      } catch(error: any) {
        console.log(error);
        console.log(Object.keys(error));
      }
    };

    getVacancies();
  }, [currentLocation]);

  return (
    <View style={styles.container}>
      {isInitialRegionSet() && (
        <MapView style={styles.map} initialRegion={initialRegion}>
          <Marker 
            key="main"
            coordinate={{
              latitude: initialRegion.latitude,
              longitude: initialRegion.longitude,
            }}
            title="Your Location"
          />
          {isCurrLocationSet() && 
            vacancies.map(vacancie => (
              <Marker
                key={vacancie.id}
                coordinate={{
                  latitude: vacancie.region.latitude,
                  longitude: vacancie.region.longitude,
                }}
                title={`Vacancy ${vacancie.id} Location`}
              />
            ))
          }
        </MapView>
      )}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    alignItems: "center",
    justifyContent: "center",
  },
  map: {
    width: "100%",
    height: "100%",
  },
});

export default Navigation;