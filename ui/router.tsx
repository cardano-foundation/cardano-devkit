import { Route, Routes } from 'react-router-dom';
import Landing from './pages/Landing';
import { ROUTES } from './constants/routes';
import Setup from './pages/Setup';

const Router = () => {
    return (
        <Routes>
            <Route path={ROUTES.LANDING} element={<Landing />} />
            <Route path={ROUTES.SETUP} element={<Setup />} />
        </Routes>
    );
};

export default Router;