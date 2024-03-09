import * as React from 'react';
import { Button } from 'react-native';

const Home = ({navigation}: {navigation:any}) => {
    console.log(navigation);

    return (
        <>
            <Button
                title="Go to map view"
                onPress={() =>
                    navigation.push('Navigation')
                }
            />
        </>
    );
}

export default Home;