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

const windowWidth = Dimensions.get("window").width;
const windowHeight = Dimensions.get("window").height;

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

      let location = await Location.getCurrentPositionAsync({});
      console.log(location);
      setCurrentLocation(location.coords);

      setInitialRegion({
        ...location.coords,
        latitudeDelta: 0.1,
        longitudeDelta: 0.1,
      });
    };

    getLocation();
  }, []);

  useEffect(() => {
    const getVacancies = async () => {
      console.log("going to call the backend")
      try {
        const response = await axios.get('http://10.0.2.2:8000/vacancies');
        console.log(response.data);
        setVacancies(response.data);
      } catch(error) {
        console.log(error);
      }
    };

    getVacancies();
  }, []);

  return (
    <View style={styles.container}>
      {isInitialRegionSet() && (
        <MapView style={styles.map} initialRegion={initialRegion}>
          {isCurrLocationSet() && 
            vacancies.map(vacancie => (
              <Marker
                key={vacancie.id}
                coordinate={{
                  latitude: vacancie.region.latitude,
                  longitude: vacancie.region.longitude,
                }}
                title="Your Location"
              />
            ))
          }
        </MapView>
      )}
      {/* Rest of your code */}
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