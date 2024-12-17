import { Route, Routes } from 'react-router-dom';
import Landing from './pages/Landing';
import { ROUTES } from './constants/routes';

const Router = () => {
    return (
        <Routes>
            <Route path={ROUTES.LANDING} element={<Landing />} />
        </Routes>
    );
};

export default Router;