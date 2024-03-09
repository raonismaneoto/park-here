import * as React from 'react';
import { NavigationContainer } from '@react-navigation/native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import Home from './components/Home';
import { Appbar, Text } from 'react-native-paper';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { View } from 'react-native';
import Navigation from './components/Navigation';


const Stack = createNativeStackNavigator();

const App = () => {
  
  return (
    <SafeAreaProvider>
      <NavigationContainer>
        <Stack.Navigator>
          <Stack.Screen 
            name="Home"
            component={Home}
          />
          <Stack.Screen 
            name="Navigation"
            component={Navigation}
          />
        </Stack.Navigator>
      </NavigationContainer>
    </SafeAreaProvider>
   
  );
};

export default App;