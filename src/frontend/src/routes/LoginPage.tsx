import { ClientLogin } from '@calimero-network/calimero-client';
import { useNavigate } from 'react-router-dom';
import { getApplicationId, getNodeUrl } from '@/utils/node';
import {
  clearApplicationIdFromLocalStorage,
  clearNodeUrlFromLocalStorage,
} from '@/utils/storage';
import ContentWrapper from '@/components/ContentWrapper';
import GetemLogo from '@/assets/getem_white.svg';
import {
  Card,
  CardContent,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card';
import { Button } from '@/components/ui/button';

export default function LoginPage() {
  const navigate = useNavigate();
  function onSetupClick() {
    clearNodeUrlFromLocalStorage();
    clearApplicationIdFromLocalStorage();
    navigate('/');
  }
  return (
    <ContentWrapper>
      <div className="relative flex h-screen w-full bg-stone-300">
        <div className="flex h-full w-full flex-col items-center justify-center">
          <Card className="">
            <CardHeader>
              <CardTitle className="flex items-center justify-center">
                <img
                  src={GetemLogo}
                  alt="Getem logo"
                  className="w-62 rounded-lg"
                />
              </CardTitle>
            </CardHeader>

            <CardContent>
              <div className="">
                <ClientLogin
                  getNodeUrl={getNodeUrl}
                  getApplicationId={getApplicationId}
                  sucessRedirect={() => navigate('/home')}
                />
              </div>
            </CardContent>

            <CardFooter className="flex justify-center">
              <Button
                onClick={onSetupClick}
                className="mt-4 rounded-lg border border-gray-600 px-6 py-2 text-gray-400 transition-all duration-200 hover:border-pink-500 hover:text-pink-500"
              >
                Return to setup
              </Button>
            </CardFooter>
          </Card>
        </div>
      </div>
    </ContentWrapper>
  );
}
