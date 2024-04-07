import * as React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import Home from './components/Home';
import { Appbar, Text, Provider } from 'react-native-paper';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { View } from 'react-native';
import Navigation from './components/Navigation';
import Login from './components/Login';
import Subscription from './components/Subscription';
import VehiclesRegistration from './components/VehiclesRegistration';


const Stack = createNativeStackNavigator();

const App = () => {
  
  return (
    <Provider>
      <SafeAreaProvider>
        <NavigationContainer>
          <Stack.Navigator>
            <Stack.Screen 
              name="Login"
              component={Login}
            />
            <Stack.Screen 
              name="Park Here"
              component={Home}
            />
            <Stack.Screen 
              name="Navigation"
              component={Navigation}
            />
            <Stack.Screen 
              name="Subscription"
              component={Subscription}
            />
            <Stack.Screen 
              name="VehiclesRegistration"
              component={VehiclesRegistration}
            />
          </Stack.Navigator>
        </NavigationContainer>
      </SafeAreaProvider>
    </Provider>
  );
};

export default App;